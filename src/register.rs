//! ADXL355 register addresses
#![allow(non_camel_case_types)]

// See datasheet https://www.analog.com/media/en/technical-documentation/data-sheets/adxl354_355.pdf

#[allow(dead_code)]
#[repr(u8)]
pub enum Register {

    DEVID_AD = 0x00,
    DEVID_MST = 0x01,
    /// Device ID (Read Only)
    ///
    /// "The DEVID register holds a fixed device ID code of 0xED (355 octal)."
    DEVID = 0x02,
    REVID = 0x03,
    STATUS = 0x04,
    FIFO_ENTRIES = 0x05,
    TEMP2 = 0x06,
    TEMP1 = 0x07,
    XDATA3 = 0x08,
    XDATA2 = 0x09,
    XDATA1 = 0x0A,
    YDATA3 = 0x0B,
    YDATA2 = 0x0C,
    YDATA1 = 0x0D,
    ZDATA3 = 0x0E,
    ZDATA2 = 0x0F,
    ZDATA1 = 0x10,
    FIFO_DATA = 0x11,
    OFFSET_X_H = 0x1E,
    OFFSET_X_L = 0x1F,
    OFFSET_Y_H = 0x20,
    OFFSET_Y_L = 0x21,
    OFFSET_Z_H = 0x22,
    OFFSET_Z_L = 0x23,
    ACT_EN = 0x24,
    ACT_THRESH_H = 0x25,
    ACT_THRESH_L = 0x26,
    ACT_COUNT =  0x27,
    FILTER = 0x28,
    FIFO_SAMPLES = 0x29,
    INT_MAP = 0x2A,
    SYNC = 0x2B,
    RANGE = 0x2C,
    POWER_CTL = 0x2D,
    SELF_TEST = 0x2E,
    RESET = 0x2F
}

impl Register {
    /// Get register address
    pub fn addr(self) -> u8 {
        self as u8
    }

}
