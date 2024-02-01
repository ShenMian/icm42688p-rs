#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum Error {
    Spi,
    I2C,
    BadDeviceId,
}
