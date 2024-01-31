#[derive(Clone, Eq, PartialEq, Debug)]
pub enum Error {
    /// Pin operation failed.
    Pin,

    /// SPI communication failed.
    Spi,

    /// Unknown device id.
    BadDeviceId,

    /// The data returned from the sensor is invalid.
    DataCorrupted,
}
