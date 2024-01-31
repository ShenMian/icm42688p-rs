use crate::error::Error;

/// Accelerometer output data rate.
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, Default)]
pub enum AccelOdr {
    /// 32 kHz (LN mode).
    Hz32k = 1,
    /// 16 kHz (LN mode).
    Hz16k = 2,
    /// 8 kHz (LN mode).
    Hz8k = 3,
    /// 4 kHz (LN mode).
    Hz4k = 4,
    /// 2 kHz (LN mode).
    Hz2k = 5,
    /// 1 kHz (LN mode).
    #[default]
    Hz1k = 6,
    /// 200 Hz (LP or LN mode).
    Hz200 = 7,
    /// 100 Hz (LP or LN mode).
    Hz100 = 8,
    /// 50 Hz (LP or LN mode).
    Hz50 = 9,
    /// 25 Hz (LP or LN mode).
    Hz25 = 10,
    /// 12.5 Hz (LP or LN mode).
    Hz12_5 = 11,
    /// 6.25 Hz (LP mode).
    Hz6_25 = 12,
    /// 3.125 Hz (LP mode).
    Hz3_125 = 13,
    /// 1.5625 Hz (LP mode).
    Hz1_5625 = 14,
    /// 500 Hz (LP or LN mode).
    Hz500 = 15,
}

impl AccelOdr {
    /// Returns true if this ODR is supported in Low Noise mode.
    pub fn supports_low_noise(&self) -> bool {
        matches!(
            self,
            Self::Hz32k
                | Self::Hz16k
                | Self::Hz8k
                | Self::Hz4k
                | Self::Hz2k
                | Self::Hz1k
                | Self::Hz200
                | Self::Hz100
                | Self::Hz50
                | Self::Hz25
                | Self::Hz12_5
                | Self::Hz500
        )
    }

    /// Returns true if this ODR is supported in Low Power mode.
    pub fn supports_low_power(&self) -> bool {
        matches!(
            self,
            Self::Hz500
                | Self::Hz200
                | Self::Hz100
                | Self::Hz50
                | Self::Hz25
                | Self::Hz12_5
                | Self::Hz6_25
                | Self::Hz3_125
                | Self::Hz1_5625
        )
    }
}

impl TryFrom<u8> for AccelOdr {
    type Error = Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(Self::Hz32k),
            2 => Ok(Self::Hz16k),
            3 => Ok(Self::Hz8k),
            4 => Ok(Self::Hz4k),
            5 => Ok(Self::Hz2k),
            6 => Ok(Self::Hz1k),
            7 => Ok(Self::Hz200),
            8 => Ok(Self::Hz100),
            9 => Ok(Self::Hz50),
            10 => Ok(Self::Hz25),
            11 => Ok(Self::Hz12_5),
            12 => Ok(Self::Hz6_25),
            13 => Ok(Self::Hz3_125),
            14 => Ok(Self::Hz1_5625),
            15 => Ok(Self::Hz500),
            _ => Err(Error::DataCorrupted),
        }
    }
}

/// Accel full-scale range.
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, Default)]
pub enum AccelRange {
    /// ±16g.
    #[default]
    G16 = 0,
    /// ±8g.
    G8 = 1,
    /// ±4g.
    G4 = 2,
    /// ±2g.
    G2 = 3,
}

impl AccelRange {
    /// Sensitivity scale factor, unit: LSB/g.
    pub fn sensitivity_scale_factor(&self) -> f32 {
        match &self {
            Self::G16 => 2048.0,
            Self::G8 => 4096.0,
            Self::G4 => 8192.0,
            Self::G2 => 16384.0,
        }
    }
}

impl TryFrom<u8> for AccelRange {
    type Error = Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::G16),
            1 => Ok(Self::G8),
            2 => Ok(Self::G4),
            3 => Ok(Self::G2),
            _ => Err(Error::DataCorrupted),
        }
    }
}

/// Gyroscope output data rate.
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, Default)]
pub enum GyroOdr {
    /// 32 kHz.
    Hz32k = 1,
    /// 16 kHz.
    Hz16k = 2,
    /// 8 kHz.
    Hz8k = 3,
    /// 4 kHz.
    Hz4k = 4,
    /// 2 kHz.
    Hz2k = 5,
    /// 1 kHz.
    #[default]
    Hz1k = 6,
    /// 200 Hz.
    Hz200 = 7,
    /// 100 Hz.
    Hz100 = 8,
    /// 50 Hz.
    Hz50 = 9,
    /// 25 Hz.
    Hz25 = 10,
    /// 12.5 Hz.
    Hz12_5 = 11,
    /// 500 Hz.
    Hz500 = 15,
}

impl TryFrom<u8> for GyroOdr {
    type Error = Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(Self::Hz32k),
            2 => Ok(Self::Hz16k),
            3 => Ok(Self::Hz8k),
            4 => Ok(Self::Hz4k),
            5 => Ok(Self::Hz2k),
            6 => Ok(Self::Hz1k),
            7 => Ok(Self::Hz200),
            8 => Ok(Self::Hz100),
            9 => Ok(Self::Hz50),
            10 => Ok(Self::Hz25),
            11 => Ok(Self::Hz12_5),
            15 => Ok(Self::Hz500),
            _ => Err(Error::DataCorrupted),
        }
    }
}

/// Gyro full-scale range.
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, Default)]
pub enum GyroRange {
    /// ±2000°/s.
    #[default]
    Dps2000 = 0,
    /// ±1000°/s.
    Dps1000 = 1,
    /// ±500°/s.
    Dps500 = 2,
    /// ±250°/s.
    Dps250 = 3,
    /// ±125°/s.
    Dps125 = 4,
    /// ±62.5°/s.
    Dps62_5 = 5,
    /// ±31.25°/s.
    Dps31_25 = 6,
    /// ±15.625°/s.
    Dps15_625 = 7,
}

impl GyroRange {
    /// Sensitivity scale factor, unit: LSB/(º/s).
    pub fn sensitivity_scale_factor(&self) -> f32 {
        match &self {
            Self::Dps2000 => 16.4,
            Self::Dps1000 => 32.8,
            Self::Dps500 => 65.5,
            Self::Dps250 => 131.0,
            Self::Dps125 => 262.0,
            Self::Dps62_5 => 524.3,
            Self::Dps31_25 => 1048.6,
            Self::Dps15_625 => 2097.2,
        }
    }
}

impl TryFrom<u8> for GyroRange {
    type Error = Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Dps2000),
            1 => Ok(Self::Dps1000),
            2 => Ok(Self::Dps500),
            3 => Ok(Self::Dps250),
            4 => Ok(Self::Dps125),
            5 => Ok(Self::Dps62_5),
            6 => Ok(Self::Dps31_25),
            7 => Ok(Self::Dps15_625),
            _ => Err(Error::DataCorrupted),
        }
    }
}

/// Gyroscope mode
///
/// Gyroscope needs to be kept ON for a minimum of 45ms.
/// When transitioning from OFF to any of the other modes, do not issue any
/// register writes for 200μs.
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, Default)]
pub enum GyroMode {
    #[default]
    Off = 0b00, // Off.
    Standby = 0b01,  // Standby mode.
    LowNoise = 0b11, // Low noise mode.
}

impl TryFrom<u8> for GyroMode {
    type Error = Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0b00 => Ok(Self::Off),
            0b01 => Ok(Self::Standby),
            0b11 => Ok(Self::LowNoise),
            _ => Err(Error::DataCorrupted),
        }
    }
}

/// Accelerometer mode
///
/// When transitioning from OFF to any of the other modes, do not issue any
/// register writes for 200μs.
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, Default)]
pub enum AccelMode {
    #[default]
    Off = 0b00, // Off.
    LowPower = 0b10, // Low power mode.
    LowNoise = 0b11, // Low noise mode.
}

impl TryFrom<u8> for AccelMode {
    type Error = Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0b00 | 0b01 => Ok(Self::Off),
            0b10 => Ok(Self::LowPower),
            0b11 => Ok(Self::LowNoise),
            _ => Err(Error::DataCorrupted),
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, Default)]
pub enum FifoMode {
    #[default]
    Bypass = 0b00, // Bypass mode.
    Stream = 0b01,     // Stream-to-FIFO mode.
    StopOnFull = 0b10, // STOP-on-FULL mode.
}

impl TryFrom<u8> for FifoMode {
    type Error = Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0b00 => Ok(Self::Bypass),
            0b01 => Ok(Self::Stream),
            0b10 | 0b11 => Ok(Self::StopOnFull),
            _ => Err(Error::DataCorrupted),
        }
    }
}

/// Standard power modes.
#[deprecated]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, Default)]
pub enum PowerMode {
    /// Gyroscope: OFF, Accelerometer: OFF
    #[default]
    Sleep = 0b0000,
    /// Gyroscope: DRIVE ON, Accelerometer: OFF
    Standby = 0b0100,
    /// Gyroscope: OFF, Accelerometer: DUTY-CYCLED
    AccelLowPower = 0b0010,
    /// Gyroscope: OFF, Accelerometer: ON
    AccelLowNoise = 0b0011,
    /// Gyroscope: ON, Accelerometer: OFF
    GyroLowNoise = 0b1100,
    /// Gyroscope: ON, Accelerometer: ON
    SixAxisLowNoise = 0b1111,
}

impl TryFrom<u8> for PowerMode {
    type Error = Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0b0000 => Ok(Self::Sleep),
            0b0100 => Ok(Self::Standby),
            0b0010 => Ok(Self::AccelLowPower),
            0b0011 => Ok(Self::AccelLowNoise),
            0b1100 => Ok(Self::GyroLowNoise),
            0b1111 => Ok(Self::SixAxisLowNoise),
            _ => Err(Error::DataCorrupted),
        }
    }
}
