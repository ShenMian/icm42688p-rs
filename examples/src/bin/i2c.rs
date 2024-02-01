#![no_main]
#![no_std]

use cortex_m_rt::entry;
use embassy_stm32::dma::NoDma;
use embassy_stm32::i2c::{self, I2c};
use embassy_stm32::time::Hertz;
use embassy_stm32::{bind_interrupts, peripherals};
use icm42688p::{I2cInterface, Icm42688p, PowerMode};
use panic_halt as _;

bind_interrupts!(struct Irqs {
    I2C1_EV => i2c::EventInterruptHandler<peripherals::I2C1>;
    I2C1_ER => i2c::ErrorInterruptHandler<peripherals::I2C1>;
});

#[entry]
fn main() -> ! {
    let peripherals = embassy_stm32::init(Default::default());

    let i2c = I2c::new(
        peripherals.I2C1,
        peripherals.PB8,
        peripherals.PB9,
        Irqs,
        NoDma,
        NoDma,
        Hertz(100_000),
        Default::default(),
    );

    let mut imu = Icm42688p::new(I2cInterface(i2c));

    imu.set_power_mode(PowerMode::SixAxisLowNoise).unwrap();

    loop {}
}
