use crate::error::Error;

/// Accelerometer output data rate
pub enum AccelODR {
    Hz32k = 1,
    Hz16k = 2,
    Hz8k = 3,
    Hz4k = 4,
    Hz2k = 5,
    Hz1k = 6,
    Hz200 = 7,
    Hz100 = 8,
    Hz50 = 9,
    Hz25 = 10,
    /// 12.5 Hz
    Hz12_5 = 11,
    /// 6.25 Hz
    Hz6_25 = 12,
    /// 3.125 Hz
    Hz3_125 = 13,
    /// 1.5625 Hz
    Hz1_5625 = 14,
    Hz500 = 15,
}

impl TryFrom<u8> for AccelODR {
    type Error = Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(AccelODR::Hz32k),
            2 => Ok(AccelODR::Hz16k),
            3 => Ok(AccelODR::Hz8k),
            4 => Ok(AccelODR::Hz4k),
            5 => Ok(AccelODR::Hz2k),
            6 => Ok(AccelODR::Hz1k),
            7 => Ok(AccelODR::Hz200),
            8 => Ok(AccelODR::Hz100),
            9 => Ok(AccelODR::Hz50),
            10 => Ok(AccelODR::Hz25),
            11 => Ok(AccelODR::Hz12_5),
            12 => Ok(AccelODR::Hz6_25),
            13 => Ok(AccelODR::Hz3_125),
            14 => Ok(AccelODR::Hz1_5625),
            15 => Ok(AccelODR::Hz500),
            _ => Err(Error::DataCorrupted),
        }
    }
}

/// Accel full-scale range
#[derive(Default)]
pub enum AccelRange {
    /// ±16g
    #[default]
    G16 = 0,
    /// ±8g
    G8 = 1,
    /// ±4g
    G4 = 2,
    /// ±2g
    G2 = 3,
}

impl AccelRange {
    /// Sensitivity scale factor, unit: LSB/g
    pub fn sensitivity_scale_factor(&self) -> f32 {
        use AccelRange as E;
        match &self {
            E::G16 => 2048.0,
            E::G8 => 4096.0,
            E::G4 => 8192.0,
            E::G2 => 16384.0,
        }
    }
}

impl TryFrom<u8> for AccelRange {
    type Error = Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        use AccelRange as E;
        match value {
            0 => Ok(E::G16),
            1 => Ok(E::G8),
            2 => Ok(E::G4),
            3 => Ok(E::G2),
            _ => Err(Error::DataCorrupted),
        }
    }
}

/// Gyroscope output data rate
pub enum GyroODR {
    Hz32k = 1,
    Hz16k = 2,
    Hz8k = 3,
    Hz4k = 4,
    Hz2k = 5,
    Hz1k = 6,
    Hz200 = 7,
    Hz100 = 8,
    Hz50 = 9,
    Hz25 = 10,
    /// 12.5 Hz
    Hz12_5 = 11,
    Hz500 = 15,
}

impl TryFrom<u8> for GyroODR {
    type Error = Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        use GyroODR as E;
        match value {
            1 => Ok(E::Hz32k),
            2 => Ok(E::Hz16k),
            3 => Ok(E::Hz8k),
            4 => Ok(E::Hz4k),
            5 => Ok(E::Hz2k),
            6 => Ok(E::Hz1k),
            7 => Ok(E::Hz200),
            8 => Ok(E::Hz100),
            9 => Ok(E::Hz50),
            10 => Ok(E::Hz25),
            11 => Ok(E::Hz12_5),
            15 => Ok(E::Hz500),
            _ => Err(Error::DataCorrupted),
        }
    }
}

/// Gyro full-scale range
#[derive(Default)]
pub enum GyroRange {
    /// ±2000°/s
    #[default]
    Dps2000 = 0,
    /// ±1000°/s
    Dps1000 = 1,
    /// ±500°/s
    Dps500 = 2,
    /// ±250°/s
    Dps250 = 3,
    /// ±125°/s
    Dps125 = 4,
    /// ±62.5°/s
    Dps62_5 = 5,
    /// ±31.25°/s
    Dps31_25 = 6,
    /// ±15.625°/s
    Dps15_625 = 7,
}

impl GyroRange {
    /// Sensitivity scale factor, unit: LSB/(º/s)
    pub fn sensitivity_scale_factor(&self) -> f32 {
        use GyroRange as E;
        match &self {
            E::Dps2000 => 16.4,
            E::Dps1000 => 32.8,
            E::Dps500 => 65.5,
            E::Dps250 => 131.0,
            E::Dps125 => 262.0,
            E::Dps62_5 => 524.3,
            E::Dps31_25 => 1048.6,
            E::Dps15_625 => 2097.2,
        }
    }
}

impl TryFrom<u8> for GyroRange {
    type Error = Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        use GyroRange as E;
        match value {
            0 => Ok(E::Dps2000),
            1 => Ok(E::Dps1000),
            2 => Ok(E::Dps500),
            3 => Ok(E::Dps250),
            4 => Ok(E::Dps125),
            5 => Ok(E::Dps62_5),
            6 => Ok(E::Dps31_25),
            7 => Ok(E::Dps15_625),
            _ => Err(Error::DataCorrupted),
        }
    }
}

/// Standard power modes
#[derive(PartialEq, Default)]
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
        use PowerMode as E;
        match value {
            0b0000 => Ok(E::Sleep),
            0b0100 => Ok(E::Standby),
            0b0010 => Ok(E::AccelLowPower),
            0b0011 => Ok(E::AccelLowNoise),
            0b1100 => Ok(E::GyroLowNoise),
            0b1111 => Ok(E::SixAxisLowNoise),
            _ => Err(Error::DataCorrupted),
        }
    }
}
