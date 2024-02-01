#![no_std]

mod config;
mod error;
mod interface;
mod register;

pub use config::*;
use error::Error;
pub use interface::*;
use register::*;

/// ICM-42688-P driver.
///
/// Orientation of axes:
///
/// +Z
/// ^   +Y head
/// | 7
/// |/
/// +-------> +X
pub struct Icm42688p<T> {
    interface: T,
}

impl<T: Interface> Icm42688p<T> {
    const DEVICE_ID: u8 = 0x47;

    pub fn new(interface: T) -> Self {
        Self { interface }
    }

    pub fn init(&mut self) -> Result<(), Error> {
        if self.device_id()? != Self::DEVICE_ID {
            return Err(Error::BadDeviceId);
        }
        self.set_accel_range(AccelRange::default())?;
        self.set_gyro_range(GyroRange::default())?;
        self.set_accel_odr(AccelOdr::default())?;
        self.set_gyro_odr(GyroOdr::default())?;
        self.set_power_mode(PowerMode::SixAxisLowNoise)
    }

    pub fn acceleration(&mut self) -> Result<(f32, f32, f32), Error> {
        let factor = self.accel_range()?.sensitivity_scale_factor();
        let (x, y, z) = self.raw_acceleration()?;
        let x = x as f32 / factor;
        let y = y as f32 / factor;
        let z = z as f32 / factor;
        Ok((x, y, z))
    }

    pub fn angular_velocity(&mut self) -> Result<(f32, f32, f32), Error> {
        let factor = self.gyro_range()?.sensitivity_scale_factor();
        let (x, y, z) = self.raw_angular_velocity()?;
        let x = x as f32 / factor;
        let y = y as f32 / factor;
        let z = z as f32 / factor;
        Ok((x, y, z))
    }

    pub fn raw_acceleration(&mut self) -> Result<(u16, u16, u16), Error> {
        let x = u16::from_be_bytes([
            self.read(Register::AccelDataX0)?,
            self.read(Register::AccelDataX1)?,
        ]);
        let y = u16::from_be_bytes([
            self.read(Register::AccelDataY0)?,
            self.read(Register::AccelDataY1)?,
        ]);
        let z = u16::from_be_bytes([
            self.read(Register::AccelDataZ0)?,
            self.read(Register::AccelDataZ1)?,
        ]);
        Ok((x, y, z))
    }

    pub fn raw_angular_velocity(&mut self) -> Result<(u16, u16, u16), Error> {
        let x = u16::from_be_bytes([
            self.read(Register::GyroDataX0)?,
            self.read(Register::GyroDataX1)?,
        ]);
        let y = u16::from_be_bytes([
            self.read(Register::GyroDataY0)?,
            self.read(Register::GyroDataY1)?,
        ]);
        let z = u16::from_be_bytes([
            self.read(Register::GyroDataZ0)?,
            self.read(Register::GyroDataZ1)?,
        ]);
        Ok((x, y, z))
    }

    pub fn raw_temperature(&mut self) -> Result<u16, Error> {
        Ok(u16::from_be_bytes([
            self.read(Register::TempData0)?,
            self.read(Register::TempData1)?,
        ]))
    }

    pub fn accel_range(&mut self) -> Result<AccelRange, Error> {
        let accel_fs_sel = self.read(Register::AccelConfig0)? >> 5;
        Ok((accel_fs_sel).try_into().unwrap())
    }

    pub fn set_accel_range(&mut self, range: AccelRange) -> Result<(), Error> {
        let accel_config0 = self.read(Register::AccelConfig0)? & 0xF0;
        self.write(Register::AccelConfig0, accel_config0 | range as u8)
    }

    pub fn gyro_range(&mut self) -> Result<GyroRange, Error> {
        let gyro_fs_sel = self.read(Register::GyroConfig0)? >> 5;
        Ok((gyro_fs_sel).try_into().unwrap())
    }

    pub fn set_gyro_range(&mut self, range: GyroRange) -> Result<(), Error> {
        let gyro_config0 = self.read(Register::GyroConfig0)? & 0xF0;
        self.write(Register::GyroConfig0, gyro_config0 | range as u8)
    }

    pub fn accel_odr(&mut self) -> Result<AccelOdr, Error> {
        let accel_odr = self.read(Register::AccelConfig0)? & 0x0F;
        Ok((accel_odr).try_into().unwrap())
    }

    pub fn set_accel_odr(&mut self, odr: AccelOdr) -> Result<(), Error> {
        let accel_config0 = self.read(Register::AccelConfig0)? & 0xF0;
        self.write(Register::GyroConfig0, accel_config0 | odr as u8)
    }

    pub fn gyro_odr(&mut self) -> Result<GyroOdr, Error> {
        let gyro_odr = self.read(Register::GyroConfig0)? & 0x0F;
        Ok((gyro_odr).try_into().unwrap())
    }

    pub fn set_gyro_odr(&mut self, odr: GyroOdr) -> Result<(), Error> {
        let gyro_config0 = self.read(Register::GyroConfig0)? & 0xF0;
        self.write(Register::GyroConfig0, gyro_config0 | odr as u8)
    }

    pub fn power_mode(&mut self) -> Result<PowerMode, Error> {
        let accel_gyro_mode = self.read(Register::PwrMgmt0)? & 0x0F;
        Ok((accel_gyro_mode).try_into().unwrap())
    }

    pub fn set_power_mode(&mut self, power_mode: PowerMode) -> Result<(), Error> {
        let pwr_mgmt = self.read(Register::PwrMgmt0)? & 0xF0;
        self.write(Register::PwrMgmt0, pwr_mgmt | power_mode as u8)
    }

    pub fn bank_selection(&mut self) -> Result<BankSelection, Error> {
        Ok(self.read(Register::RegBankSel)?.try_into().unwrap())
    }

    pub fn set_bank_selection(&mut self, bank_selection: BankSelection) -> Result<(), Error> {
        self.write(Register::RegBankSel, bank_selection as u8)
    }

    fn device_id(&mut self) -> Result<u8, Error> {
        self.read(Register::WhoAmI)
    }

    fn read(&mut self, register: Register) -> Result<u8, Error> {
        if register != Register::RegBankSel && self.bank_selection()? != register.bank_selection() {
            self.set_bank_selection(register.bank_selection())?;
        }
        self.interface.read_register(register.address())
    }

    fn write(&mut self, register: Register, buffer: u8) -> Result<(), Error> {
        if register != Register::RegBankSel && self.bank_selection()? != register.bank_selection() {
            self.set_bank_selection(register.bank_selection())?;
        }
        self.interface.write_register(register.address(), buffer)
    }
}
