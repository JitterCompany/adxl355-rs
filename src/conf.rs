#![allow(non_camel_case_types)]

#[derive(Copy, Clone, Debug)]
pub enum Range {
    _2G = 0b01,
    _4G = 0b10,
    _8G = 0b11,
}

impl Range {
    pub fn val(self) -> u8 {
        self as u8
    }
}

impl Default for Range {
    fn default() -> Self {
        Range::_2G
    }
}

#[derive(Copy, Clone, Debug)]
pub enum ODR_LPF {
    ODR_4000_Hz                = 0,
    ODR_2000_Hz                = 1,
    ODR_1000_Hz                = 2,
    ODR_500_Hz                 = 3,
    ODR_250_Hz                 = 4,
    ODR_125_Hz                 = 5,
    ODR_62_5_Hz                = 6,
    ODR_31_25_Hz               = 7,
    ODR_15_625_Hz              = 8,
    ODR_7_813_Hz               = 9,
    ODR_3_906_Hz               = 10,
}

impl ODR_LPF {
    pub fn val(self) -> u8 {
        self as u8
    }
}

impl Default for ODR_LPF {
    fn default() -> Self {
        ODR_LPF::ODR_3_906_Hz
    }
}

pub struct Config {
    pub(crate) range: Option<Range>,
    pub(crate) filter: Option<ODR_LPF>,
}

impl Config {

    pub fn new() -> Self {
        Config {
            range: None,
            filter: None,
        }
    }

    pub fn range(&mut self, range: Range) -> &mut Self {
        self.range = Some(range);
        self
    }

    pub fn filter(&mut self, filter: ODR_LPF) -> &mut Self {
        self.filter = Some(filter);
        self
    }
}
