// Register bank selection
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum BankSelection {
    Bank0 = 0,
    Bank1 = 1,
    Bank2 = 2,
    Bank3 = 3,
    Bank4 = 4,
}

impl TryFrom<u8> for BankSelection {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        use BankSelection::*;
        match value {
            0 => Ok(Bank0),
            1 => Ok(Bank1),
            2 => Ok(Bank2),
            3 => Ok(Bank3),
            4 => Ok(Bank4),
            _ => Err(()),
        }
    }
}

#[allow(dead_code)]
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum Register {
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

impl Register {
    pub fn address(&self) -> u8 {
        *self as u8
    }

    pub fn bank_selection(&self) -> BankSelection {
        use BankSelection::*;
        use Register::*;
        match *self {
            DeviceConfig => Bank0,
            IntConfig => Bank0,
            FifoConfig => Bank0,
            TempData1 => Bank0,
            TempData0 => Bank0,
            AccelDataX1 => Bank0,
            AccelDataX0 => Bank0,
            AccelDataY1 => Bank0,
            AccelDataY0 => Bank0,
            AccelDataZ1 => Bank0,
            AccelDataZ0 => Bank0,
            GyroDataX1 => Bank0,
            GyroDataX0 => Bank0,
            GyroDataY1 => Bank0,
            GyroDataY0 => Bank0,
            GyroDataZ1 => Bank0,
            GyroDataZ0 => Bank0,
            IntStatus => Bank0,
            FifoCountH => Bank0,
            FifoCountL => Bank0,
            FifoData => Bank0,
            SignalPathReset => Bank0,
            IntfConfig0 => Bank0,
            IntfConfig1 => Bank0,
            PwrMgmt0 => Bank0,
            GyroConfig0 => Bank0,
            AccelConfig0 => Bank0,
            GyroConfig1 => Bank0,
            GyroAccelConfig0 => Bank0,
            AccelConfig1 => Bank0,
            TmstConfig => Bank0,
            FifoConfig1 => Bank0,
            FifoConfig2 => Bank0,
            FifoConfig3 => Bank0,
            IntConfig0 => Bank0,
            IntConfig1 => Bank0,
            IntSource0 => Bank0,
            SelfTestConfig => Bank0,
            WhoAmI => Bank0,
            RegBankSel => Bank0,
        }
    }
}
