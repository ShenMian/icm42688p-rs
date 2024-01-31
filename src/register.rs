pub trait Register {
    fn address(&self) -> u8;
    fn bank(&self) -> Bank;
    fn readable(&self) -> bool;
    fn writable(&self) -> bool;
}

// Register bank selection
#[derive(PartialEq, Default)]
pub enum Bank {
    /// Bank 0: Core configuration registers
    /// - Contains main control registers
    /// - Data registers
    /// - FIFO control
    #[default]
    Bank0 = 0,

    /// Bank 1: Gyroscope configuration registers
    /// - Contains gyroscope static settings
    /// - I3C/SPI interface configuration
    Bank1 = 1,

    /// Bank 2: Accelerometer configuration registers
    /// - Contains accelerometer anti-aliasing filter settings
    Bank2 = 2,

    /// Bank 3: Clock divider configuration
    /// - Contains CLKDIV register
    /// - Used for clock frequency division settings
    Bank3 = 3,

    /// Bank 4: Advanced features configuration
    /// - Contains APEX features (pedometer, tilt detection)
    /// - TAP detection settings
    Bank4 = 4,
}

#[derive(Clone, Copy)]
pub enum Bank0 {
    DeviceConfig = 0x11,

    IntConfig = 0x14,

    FifoConfig = 0x16,

    TempData1 = 0x1D,
    TempData0 = 0x1E,

    AccelDataX1 = 0x1F,
    AccelDataX0 = 0x20,
    AccelDataY1 = 0x21,
    AccelDataY0 = 0x22,
    AccelDataZ1 = 0x23,
    AccelDataZ0 = 0x24,

    GyroDataX1 = 0x25,
    GyroDataX0 = 0x26,
    GyroDataY1 = 0x27,
    GyroDataY0 = 0x28,
    GyroDataZ1 = 0x29,
    GyroDataZ0 = 0x2A,

    IntStatus = 0x2D,
    FifoCountH = 0x2E,
    FifoCountL = 0x2F,
    FifoData = 0x30,

    SignalPathReset = 0x4B,
    IntfConfig0 = 0x4C,
    IntfConfig1 = 0x4D,
    PwrMgmt0 = 0x4E,
    GyroConfig0 = 0x4F,
    AccelConfig0 = 0x50,
    GyroConfig1 = 0x51,
    GyroAccelConfig0 = 0x52,
    AccelConfig1 = 0x53,
    TmstConfig = 0x54,

    FifoConfig1 = 0x5F,
    FifoConfig2 = 0x60,
    FifoConfig3 = 0x61,

    IntConfig0 = 0x63,
    IntConfig1 = 0x64,

    IntSource0 = 0x65,

    SelfTestConfig = 0x70,

    WhoAmI = 0x75,
    RegBankSel = 0x76,
}

impl Register for Bank0 {
    fn address(&self) -> u8 {
        *self as u8
    }

    fn bank(&self) -> Bank {
        Bank::Bank0
    }

    fn readable(&self) -> bool {
        matches!(
            self,
            Bank0::DeviceConfig
                | Bank0::FifoConfig
                | Bank0::TempData1
                | Bank0::TempData0
                | Bank0::AccelDataX1
                | Bank0::AccelDataX0
                | Bank0::AccelDataY1
                | Bank0::AccelDataY0
                | Bank0::AccelDataZ1
                | Bank0::AccelDataZ0
                | Bank0::GyroDataX1
                | Bank0::GyroDataX0
                | Bank0::GyroDataY1
                | Bank0::GyroDataY0
                | Bank0::GyroDataZ1
                | Bank0::GyroDataZ0
                | Bank0::FifoCountH
                | Bank0::FifoCountL
        )
    }

    fn writable(&self) -> bool {
        matches!(
            self,
            Bank0::DeviceConfig
                | Bank0::FifoConfig
                | Bank0::PwrMgmt0
                | Bank0::GyroConfig0
                | Bank0::AccelConfig0
                | Bank0::GyroConfig1
                | Bank0::GyroAccelConfig0
                | Bank0::AccelConfig1
        )
    }
}

pub enum Bank1 {
    GyroConfigStatic2 = 0x0B,
    GyroConfigStatic3 = 0x0C,
    GyroConfigStatic4 = 0x0D,
    GyroConfigStatic5 = 0x0E,

    IntfConfig5 = 0x7B,
}

pub enum Bank2 {
    AccelConfigStatic2 = 0x03,
    AccelConfigStatic3 = 0x04,
    AccelConfigStatic4 = 0x05,
}

pub struct BitRange {
    pub offset: u8,
    pub length: u8,
}

impl BitRange {
    pub fn mask(&self) -> u8 {
        debug_assert!(self.length >= 1);
        let mut mask: u8 = 0;
        for i in self.offset..self.offset + self.length {
            mask &= 1 << i;
        }
        mask
    }
}

#[allow(non_camel_case_types)]
pub struct ACCEL_CONFIG0;

impl ACCEL_CONFIG0 {
    /// Accelerometer ODR selection for UI interface output.
    pub const ODR: BitRange = BitRange {
        offset: 0,
        length: 4,
    };
    /// Full scale select for accelerometer UI interface output.
    pub const FS_SEL: BitRange = BitRange {
        offset: 5,
        length: 3,
    };
}

#[allow(non_camel_case_types)]
pub struct GYRO_CONFIG0;

impl GYRO_CONFIG0 {
    /// Gyroscope ODR selection for UI interface output.
    pub const ODR: BitRange = BitRange {
        offset: 0,
        length: 4,
    };
    /// Full scale select for gyroscope UI interface output.
    pub const FS_SEL: BitRange = BitRange {
        offset: 5,
        length: 3,
    };
}

#[allow(non_camel_case_types)]
pub struct FIFO_CONFIG0;

impl GYRO_CONFIG0 {
    /// Enable accelerometer packets to go to FIFO.
    pub const FIFO_ACCEL_EN: u8 = 0;
    /// Enable gyroscope packets to go to FIFO.
    pub const FIFO_GYRO_EN: u8 = 1;
    /// Enable temperature sensor packets to go to FIFO.
    pub const FIFO_TEMP_EN: u8 = 2;

    /// Must be set to 1 for all FIFO use cases when FSYNC is used.
    pub const FIFO_TMST_FSYNC_EN: u8 = 3;
    /// Enable 3 bytes of extended 20-bits accel, gyro data + 1 byte of extended
    /// 16-bit temperature sensor data to be placed into the FIFO.
    pub const FIFO_HIRES_EN: u8 = 4;
    /// Trigger FIFO watermark interrupt on every ODR (DMA write) if FIFO_COUNT
    /// ≥ FIFO_WM_TH.
    pub const FIFO_WM_GT_TH: u8 = 5;
    /// 0: Partial FIFO read disabled, requires re-reading of the entire FIFO.
    /// 1: FIFO read can be partial, and resume from last read point.
    pub const FIFO_RESUME_PARTIAL_RD: u8 = 6;
}

#[allow(non_camel_case_types)]
pub struct SIGNAL_PATH_RESET;

impl SIGNAL_PATH_RESET {
    /// When this bit is set to 1, the DMP is enabled.
    pub const DMP_INIT_EN: u8 = 1 << 6;
    /// When this bit is set to 1, the DMP memory is reset.
    pub const DMP_MEM_RESET_EN: u8 = 1 << 5;
    /// When this bit is set to 1, the signal path is reset by restarting the
    /// ODR counter and signal path controls.
    pub const ABORT_AND_RESET: u8 = 1 << 3;
    /// When this bit is set to 1, the time stamp counter is latched into the
    /// time stamp register. This is a write on clear bit.
    pub const TMST_STROBE: u8 = 1 << 2;
    /// When set to 1, FIFO will get flushed.
    pub const FIFO_FLUSH: u8 = 1 << 1;
}
