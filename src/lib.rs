#![no_std]
#![allow(dead_code)]

use core::fmt::Debug;
use embedded_hal::{digital::OutputPin, spi};

mod config;
mod error;
mod fifo;
mod register;

pub use config::*;
use error::*;
use register::*;

const ICM42688P_DEVICE_ID: u8 = 0x47;

const SPI_WRITE_OPERATION: u8 = 0b00000000;
const SPI_READ_OPERATION: u8 = 0b10000000;

/// ICM-42688-P driver.
///
/// Host interfaces:
/// - I3C: Maximum 12.5 MHz.
/// - I2C: Maximum 1 MHz.
/// - SPI: Maximum 24 MHz.
///
/// Orientation of axes:
/// +Z
/// ^   +Y head
/// | 7
/// |/
/// +-------> +X
pub struct Icm42688p<SPI, CS> {
    spi: SPI,
    cs: CS,
}

impl<EA, EB, SPI, CS> Icm42688p<SPI, CS>
where
    EA: Debug,
    EB: Debug,
    SPI: spi::SpiBus<u8, Error = EA>,
    CS: OutputPin<Error = EB>,
{
    pub fn new(spi: SPI, cs: CS) -> Result<Self, Error> {
        let mut instance = Self { spi, cs };
        if instance.device_id()? != ICM42688P_DEVICE_ID {
            return Err(Error::BadDeviceId);
        }
        Ok(instance)
    }

    pub fn acceleration(&mut self) -> Result<(f32, f32, f32), Error> {
        let (x, y, z) = self.raw_acceleration()?;
        let factor = self.accel_range()?.sensitivity_scale_factor();
        let x = x as f32 / factor;
        let y = y as f32 / factor;
        let z = z as f32 / factor;
        Ok((x, y, z))
    }

    pub fn angular_velocity(&mut self) -> Result<(f32, f32, f32), Error> {
        let (x, y, z) = self.raw_angular_velocity()?;
        // let factor =
        //     (core::f32::consts::PI / 180.0) /
        // self.gyro_range()?.sensitivity_scale_factor();
        let factor = self.gyro_range()?.sensitivity_scale_factor();
        let x = x as f32 / factor;
        let y = y as f32 / factor;
        let z = z as f32 / factor;
        Ok((x, y, z))
    }

    pub fn temperature_celsius(&mut self) -> Result<f32, Error> {
        let temp = self.raw_temperature()? as f32;
        Ok((temp / 132.48) + 25.0)
    }

    pub fn temperature_fahrenheit(&mut self) -> Result<f32, Error> {
        Ok(self.temperature_celsius()? * 1.8 + 32.0)
    }

    pub fn set_accel_range(&mut self, range: AccelRange) -> Result<(), Error> {
        let fs_sel = range as u8;
        self.set_register_bits(&Bank0::AccelConfig0, &ACCEL_CONFIG0::FS_SEL, fs_sel)
    }

    pub fn accel_range(&mut self) -> Result<AccelRange, Error> {
        let range = self.read_register(&Bank0::AccelConfig0)? >> 5;
        range.try_into()
    }

    pub fn accel_odr(&mut self) -> Result<AccelOdr, Error> {
        let odr = self.read_register(&Bank0::AccelConfig0)? & 0x0F;
        odr.try_into()
    }

    pub fn set_gyro_range(&mut self, range: GyroRange) -> Result<(), Error> {
        let fs_sel = range as u8;
        self.set_register_bits(&Bank0::GyroConfig0, &GYRO_CONFIG0::FS_SEL, fs_sel)
    }

    pub fn gyro_range(&mut self) -> Result<GyroRange, Error> {
        let range = self.read_register(&Bank0::GyroConfig0)? >> 5;
        range.try_into()
    }

    pub fn gyro_odr(&mut self) -> Result<GyroOdr, Error> {
        let odr = self.read_register(&Bank0::AccelConfig0)? & 0x0F;
        odr.try_into()
    }

    #[deprecated]
    pub fn set_power_mode(&mut self, mode: PowerMode) -> Result<(), Error> {
        let mut accel_gyro_mode = self.read_register(&Bank0::PwrMgmt0)? & 0xF0;
        accel_gyro_mode |= mode as u8;
        self.write_register(&Bank0::PwrMgmt0, accel_gyro_mode)
    }

    pub fn power_mode(&mut self) -> Result<PowerMode, Error> {
        let mode = self.read_register(&Bank0::PwrMgmt0)? & 0x0F;
        mode.try_into()
    }

    fn raw_acceleration(&mut self) -> Result<(u16, u16, u16), Error> {
        let x = u16::from_be_bytes([
            self.read_register(&Bank0::AccelDataX0)?,
            self.read_register(&Bank0::AccelDataX1)?,
        ]);
        let y = u16::from_be_bytes([
            self.read_register(&Bank0::AccelDataY0)?,
            self.read_register(&Bank0::AccelDataY1)?,
        ]);
        let z = u16::from_be_bytes([
            self.read_register(&Bank0::AccelDataZ0)?,
            self.read_register(&Bank0::AccelDataZ1)?,
        ]);
        Ok((x, y, z))
    }

    fn raw_angular_velocity(&mut self) -> Result<(u16, u16, u16), Error> {
        let x = u16::from_be_bytes([
            self.read_register(&Bank0::GyroDataX0)?,
            self.read_register(&Bank0::GyroDataX1)?,
        ]);
        let y = u16::from_be_bytes([
            self.read_register(&Bank0::GyroDataY0)?,
            self.read_register(&Bank0::GyroDataY1)?,
        ]);
        let z = u16::from_be_bytes([
            self.read_register(&Bank0::GyroDataZ0)?,
            self.read_register(&Bank0::GyroDataZ1)?,
        ]);
        Ok((x, y, z))
    }

    fn raw_temperature(&mut self) -> Result<u16, Error> {
        Ok(u16::from_be_bytes([
            self.read_register(&Bank0::TempData0)?,
            self.read_register(&Bank0::TempData1)?,
        ]))
    }

    fn fifo_count(&mut self) -> Result<u16, Error> {
        Ok(u16::from_be_bytes([
            self.read_register(&Bank0::FifoCountL)?,
            self.read_register(&Bank0::FifoCountH)?,
        ]))
    }

    fn reset_fifo(&mut self) {
        self.set_register_bits(
            &Bank0::SignalPathReset,
            &BitRange {
                offset: SIGNAL_PATH_RESET::FIFO_FLUSH,
                length: 1,
            },
            1,
        )
        .unwrap();
    }

    fn set_register_bits(
        &mut self,
        reg: &dyn Register,
        range: &BitRange,
        data: u8,
    ) -> Result<(), Error> {
        let mut buf = self.read_register(reg)?;
        buf &= !range.mask();
        buf |= data << range.offset;
        self.write_register(reg, buf)
    }

    fn read_register(&mut self, reg: &dyn Register) -> Result<u8, Error> {
        debug_assert!(reg.readable());
        let mut buf: [u8; 2] = [reg.address() | SPI_READ_OPERATION, 0];
        self.spi
            .transfer_in_place(&mut buf)
            .map_err(|_| Error::Spi)?;
        Ok(buf[1])
    }

    fn write_register(&mut self, reg: &dyn Register, data: u8) -> Result<(), Error> {
        debug_assert!(reg.writable());
        let mut buf: [u8; 2] = [reg.address() | SPI_WRITE_OPERATION, data];
        self.spi
            .transfer_in_place(&mut buf)
            .map_err(|_| Error::Spi)?;
        Ok(())
    }

    fn select_bank(&mut self, _reg: &dyn Register, bank: Bank) -> Result<(), Error> {
        self.write_register(&Bank0::RegBankSel, bank as u8)
    }

    pub fn device_id(&mut self) -> Result<u8, Error> {
        self.read_register(&Bank0::WhoAmI)
    }

    fn select_chip(&mut self) -> Result<(), Error> {
        self.cs.set_low().map_err(|_| Error::Pin)
    }

    fn unselect_chip(&mut self) -> Result<(), Error> {
        self.cs.set_high().map_err(|_| Error::Pin)
    }
}
