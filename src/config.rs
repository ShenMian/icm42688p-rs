/// Accelerometer output data rate
#[derive(Copy, Clone, Eq, PartialEq, Debug, Default)]
pub enum AccelOdr {
    Hz32k = 1,
    Hz16k = 2,
    Hz8k = 3,
    Hz4k = 4,
    Hz2k = 5,
    #[default]
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

impl TryFrom<u8> for AccelOdr {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        use AccelOdr::*;
        match value {
            1 => Ok(Hz32k),
            2 => Ok(Hz16k),
            3 => Ok(Hz8k),
            4 => Ok(Hz4k),
            5 => Ok(Hz2k),
            6 => Ok(Hz1k),
            7 => Ok(Hz200),
            8 => Ok(Hz100),
            9 => Ok(Hz50),
            10 => Ok(Hz25),
            11 => Ok(Hz12_5),
            12 => Ok(Hz6_25),
            13 => Ok(Hz3_125),
            14 => Ok(Hz1_5625),
            15 => Ok(Hz500),
            _ => Err(()),
        }
    }
}

/// Accel full-scale range
#[derive(Copy, Clone, Eq, PartialEq, Debug, Default)]
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
        use AccelRange::*;
        match &self {
            G16 => 2048.0,
            G8 => 4096.0,
            G4 => 8192.0,
            G2 => 16384.0,
        }
    }
}

impl TryFrom<u8> for AccelRange {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        use AccelRange::*;
        match value {
            0 => Ok(G16),
            1 => Ok(G8),
            2 => Ok(G4),
            3 => Ok(G2),
            _ => Err(()),
        }
    }
}

/// Gyroscope output data rate
#[derive(Copy, Clone, Eq, PartialEq, Debug, Default)]
pub enum GyroOdr {
    Hz32k = 1,
    Hz16k = 2,
    Hz8k = 3,
    Hz4k = 4,
    Hz2k = 5,
    #[default]
    Hz1k = 6,
    Hz200 = 7,
    Hz100 = 8,
    Hz50 = 9,
    Hz25 = 10,
    /// 12.5 Hz
    Hz12_5 = 11,
    Hz500 = 15,
}

impl TryFrom<u8> for GyroOdr {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        use GyroOdr::*;
        match value {
            1 => Ok(Hz32k),
            2 => Ok(Hz16k),
            3 => Ok(Hz8k),
            4 => Ok(Hz4k),
            5 => Ok(Hz2k),
            6 => Ok(Hz1k),
            7 => Ok(Hz200),
            8 => Ok(Hz100),
            9 => Ok(Hz50),
            10 => Ok(Hz25),
            11 => Ok(Hz12_5),
            15 => Ok(Hz500),
            _ => Err(()),
        }
    }
}

/// Gyro full-scale range
#[derive(Copy, Clone, Eq, PartialEq, Debug, Default)]
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
        use GyroRange::*;
        match &self {
            Dps2000 => 16.4,
            Dps1000 => 32.8,
            Dps500 => 65.5,
            Dps250 => 131.0,
            Dps125 => 262.0,
            Dps62_5 => 524.3,
            Dps31_25 => 1048.6,
            Dps15_625 => 2097.2,
        }
    }
}

impl TryFrom<u8> for GyroRange {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        use GyroRange::*;
        match value {
            0 => Ok(Dps2000),
            1 => Ok(Dps1000),
            2 => Ok(Dps500),
            3 => Ok(Dps250),
            4 => Ok(Dps125),
            5 => Ok(Dps62_5),
            6 => Ok(Dps31_25),
            7 => Ok(Dps15_625),
            _ => Err(()),
        }
    }
}

/// Standard power modes
#[derive(Copy, Clone, Eq, PartialEq, Debug, Default)]
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
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        use PowerMode::*;
        match value {
            0b0000 => Ok(Sleep),
            0b0100 => Ok(Standby),
            0b0010 => Ok(AccelLowPower),
            0b0011 => Ok(AccelLowNoise),
            0b1100 => Ok(GyroLowNoise),
            0b1111 => Ok(SixAxisLowNoise),
            _ => Err(()),
        }
    }
}
