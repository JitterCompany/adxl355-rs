#![no_std]

mod conf;
mod register;

use embedded_hal as hal;
use micromath::vector::I32x3;

use hal::blocking::spi;
use hal::digital::v2::OutputPin;

pub use conf::*;

use register::Register;

const SPI_READ: u8 = 0x01;
const SPI_WRITE: u8 = 0x00;

const EXPECTED_DEVICE_ID: u8 = 0xED;

/// ADXL355 driver
pub struct Adxl355<SPI, CS> {
    spi: SPI,
    cs: CS,

    // configuration
    odr: ODR_LPF,
    range: Range
}



impl<SPI, CS, E> Adxl355<SPI, CS>
where
    SPI: spi::Transfer<u8, Error=E> + spi::Write<u8, Error=E>,
    CS: OutputPin<Error = ()>,
{

    /// Creates a new [`adxl355`] driver from a SPI peripheral with
    /// default configuration.
    pub fn default(spi:SPI, cs:CS) -> Result<Self, E> {
        Adxl355::new(spi, cs, &Config::new())
    }

    pub fn new(spi:SPI, cs:CS, config: &Config) -> Result<Self, E> {
        let mut adxl355 = Adxl355 {
            spi,
            cs,
            odr: config.filter.unwrap_or_default(),
            range: config.range.unwrap_or_default()
        };


        let id = adxl355.get_device_id();

        if id != EXPECTED_DEVICE_ID {
            // error

        }

        adxl355.write_reg(Register::FILTER.addr(), adxl355.odr.val());
        adxl355.write_reg(Register::RANGE.addr(), adxl355.range.val());

        Ok(adxl355)
    }

    pub fn start(&mut self) {
        self.write_reg(Register::POWER_CTL.addr(), 0);
    }


    pub fn read_temp(&mut self) -> u16 {

        let mut bytes = [(Register::TEMP2.addr() << 1)  | SPI_READ, 0, 0];
        self.read(&mut bytes);

        let temp_h = ((bytes[1] & 0x0F) as u16) << 8;
        let temp_l = (bytes[2] as u16) & 0x00FF;

        temp_h | temp_l
    }

    pub fn acceleration(&mut self) -> I32x3 {
        let mut bytes = [0u8; 9+1];
        bytes[0] = (Register::XDATA3.addr() << 1)  | SPI_READ;
        self.read(&mut bytes);

        // combine 3 bytes into one i32 value
        // right-shift with sign-extend to 20-bit
        let x = ((((bytes[1] as i32) << 24) | ((bytes[2] as i32) << 16) | ((bytes[3] & 0xF0) as i32) << 8)) >> 12;
        let y = ((((bytes[4] as i32) << 24) | ((bytes[5] as i32) << 16) | ((bytes[6] & 0xF0) as i32) << 8)) >> 12;
        let z = ((((bytes[7] as i32) << 24) | ((bytes[8] as i32) << 16) | ((bytes[9] & 0xF0) as i32) << 8)) >> 12;

        I32x3::new(x, y, z)
    }

    /// Get the device ID
    pub fn get_device_id(&mut self) -> u8 {
        let reg = Register::DEVID.addr();
        let mut output = [1u8];
        self.read_reg(reg, &mut output);
        output[0]
    }

    fn write_reg(&mut self, reg: u8, value: u8) {
        let mut bytes = [(reg << 1)  | SPI_WRITE, value];
        self.cs.set_low().unwrap();
        self.spi.write(&mut bytes).ok();
        self.cs.set_high().unwrap();
    }

    fn read_reg(&mut self, reg: u8, buffer: &mut [u8]) {
        let mut bytes = [(reg << 1)  | SPI_READ, 0];
        self.cs.set_low().unwrap();
        self.spi.transfer(&mut bytes).ok();
        self.cs.set_high().unwrap();
        buffer[0] = bytes[1];
    }

    fn read(&mut self, bytes: &mut [u8]) {
        self.cs.set_low().unwrap();
        self.spi.transfer(bytes).ok();
        self.cs.set_high().unwrap();
    }
}



#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
