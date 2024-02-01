#![no_main]
#![no_std]

use cortex_m_rt::entry;
use embassy_stm32::dma::NoDma;
use embassy_stm32::gpio::{Level, Output, Speed};
use embassy_stm32::spi::Spi;
use icm42688p::{Icm42688p, PowerMode, SpiBusInterface};
use panic_halt as _;

#[entry]
fn main() -> ! {
    let peripherals = embassy_stm32::init(Default::default());

    let spi = Spi::new(
        peripherals.SPI1,
        peripherals.PA5,
        peripherals.PA7,
        peripherals.PA6,
        NoDma,
        NoDma,
        Default::default(),
    );
    let mut cs = Output::new(peripherals.PE0, Level::High, Speed::VeryHigh);

    let mut imu = Icm42688p::new(SpiBusInterface(spi));

    cs.set_low();
    imu.set_power_mode(PowerMode::SixAxisLowNoise).unwrap();
    cs.set_high();

    loop {}
}
