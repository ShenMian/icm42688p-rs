use crate::error::Error;
use embedded_hal::i2c::{self, I2c};
use embedded_hal::spi::SpiBus;

pub trait Interface {
    fn read_register(&mut self, address: u8) -> Result<u8, Error>;
    fn write_register(&mut self, address: u8, buffer: u8) -> Result<(), Error>;
}

pub struct SpiBusInterface<SPI>(pub SPI);

impl<SPI> Interface for SpiBusInterface<SPI>
where
    SPI: SpiBus,
{
    fn read_register(&mut self, address: u8) -> Result<u8, Error> {
        // SPI read operations are completed in 16 or more clock cycles (two or more
        // bytes). The first byte contains the SPI Address, and the following byte(s)
        // contain(s) the SPI data. The first bit of the first byte contains the
        // Read/Write bit and indicates the Read (1) operation. The following 7 bits
        // contain the Register Address. In cases of multiple-byte Reads, data is two or
        // more bytes
        debug_assert!(
            address & 0b10000000 == 0,
            "the MSB of the address should be 0"
        );
        const READ: u8 = 0b10000000;
        let tx_buffer = [READ | address];
        let mut rx_buffer = [0u8];
        self.0
            .transfer(&mut rx_buffer, &tx_buffer)
            .map_err(|_| Error::Spi)?;
        Ok(rx_buffer[0])
    }

    fn write_register(&mut self, address: u8, buffer: u8) -> Result<(), Error> {
        // SPI write operations are completed in 16 clock cycles (two bytes). The first
        // byte contains the SPI Address, and the second byte contains the SPI data. The
        // first bit of the first byte contains the Read/Write bit and indicates the
        // Write (0) operation. The following 7 bits contain the Register Address.
        debug_assert!(
            address & 0b10000000 == 0,
            "the MSB of the address should be 0"
        );
        const WRITE: u8 = 0b00000000;
        let tx_buffer = [WRITE | address, buffer];
        self.0.write(&tx_buffer).map_err(|_| Error::Spi)?;
        Ok(())
    }
}

pub struct I2cInterface<I2C>(pub I2C);

impl<I2C> I2cInterface<I2C> {
    const SLAVE_ADDRESS: u8 = 0b1101000;
}

impl<I2C> Interface for I2cInterface<I2C>
where
    I2C: I2c,
{
    fn read_register(&mut self, address: u8) -> Result<u8, Error> {
        // To read the internal ICM-42688-P registers, the master sends a start
        // condition, followed by the I2C address and a write bit, and then the register
        // address that is going to be read. Upon receiving the ACK signal from the
        // ICM-42688-P, the master transmits a start signal followed by the slave
        // address and read bit. As a result, the ICM-42688-P sends an ACK signal and
        // the data. The communication ends with a not acknowledge (NACK) signal and a
        // stop bit from master.
        let mut rx_buffer = [0u8];
        self.0
            .transaction(
                Self::SLAVE_ADDRESS,
                &mut [
                    i2c::Operation::Write(&[address]),
                    i2c::Operation::Read(&mut rx_buffer),
                ],
            )
            .map_err(|_| Error::I2C)?;
        Ok(rx_buffer[0])
    }

    fn write_register(&mut self, address: u8, buffer: u8) -> Result<(), Error> {
        // To write the internal ICM-42688-P registers, the master transmits the start
        // condition (S), followed by the I2C address and the write bit (0). At the 9th
        // clock cycle (when the clock is high), the ICM-42688-P acknowledges the
        // transfer. Then the master puts the register address (RA) on the bus. After
        // the ICM-42688-P acknowledges the reception of the register address, the
        // master puts the register data onto the bus. This is followed by the ACK
        // signal, and data transfer may be concluded by the stop condition (P).
        self.0
            .transaction(
                Self::SLAVE_ADDRESS,
                &mut [i2c::Operation::Write(&[address, buffer])],
            )
            .map_err(|_| Error::I2C)?;
        Ok(())
    }
}
