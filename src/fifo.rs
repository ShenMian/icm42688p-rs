use bitflags::bitflags;

bitflags! {
    #[derive(Clone, Copy, PartialEq, Eq, Debug)]
    pub struct HeaderFlags: u8 {
        ///  FIFO is empty
        const FifoEmpty = 1 << 7;
        /// Packet is sized so that accel data have location in the packet, FIFO_ACCEL_EN must be 1
        const ContainsAccelData = 1 << 6;
        /// Packet is sized so that gyro data have location in the packet, FIFO_GYRO_EN must be 1
        const ContainsGyroData = 1 << 5;
        /// Packet contains ODR Timestamp
        const ContainsODRTimestamp = 0b10 << 2;
        /// Packet contains FSYNC time, and this packet is flagged as first ODR after FSYNC (only if FIFO_TMST_FSYNC_EN is 1)
        const ContainsFsyncTime = 0b11 << 2;
        ///  The ODR for accel is different for this accel data packet compared to the previous accel packet
        const AccelODRDiff = 1 << 1;
        /// The ODR for gyro is different for this gyro data packet compared to the previous gyro packet
        const GyroODRDiff = 1 << 0;
    }
}

struct FifoPacket {
    header: u8,
    accel_x: u16,
    accel_y: u16,
    accel_z: u16,
    gyro_x: u16,
    gyro_y: u16,
    gyro_z: u16,
    temperature: u16,
    timestamp: u16,
    ext_accel_x_gyro_x: u8,
    ext_accel_y_gyro_y: u8,
    ext_accel_z_gyro_z: u8,
}

impl FifoPacket {
    pub fn accelerometer(&self) -> Option<(u16, u16, u16)> {
        if self.header & HeaderFlags::ContainsAccelData.bits() != 0 {
            Some((self.accel_x, self.accel_y, self.accel_z))
        } else {
            None
        }
    }

    pub fn gyroscope(&self) -> Option<(u16, u16, u16)> {
        if self.header & HeaderFlags::ContainsGyroData.bits() != 0 {
            Some((self.gyro_x, self.gyro_y, self.gyro_z))
        } else {
            None
        }
    }
}
