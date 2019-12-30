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

#[derive(Copy, Clone, Debug)]
pub enum HPF_CORNER {
    NONE        = 0,
    _247_ODR    = 1,
    _62_084_ODR = 2,
    _15_545_ODR = 3,
    _3_862_ODR  = 4,
    _0_954_ODR  = 5,
    _0_238_ODR  = 6
}

impl HPF_CORNER {
    pub fn val(self) -> u8 {
        self as u8
    }
}

impl Default for HPF_CORNER {
    fn default() -> Self {
        HPF_CORNER::NONE
    }
}

pub struct Config {
    pub(crate) range: Option<Range>,
    pub(crate) odr: Option<ODR_LPF>,
    pub(crate) hpf: Option<HPF_CORNER>
}

impl Config {

    pub fn new() -> Self {
        Config {
            range: None,
            odr: None,
            hpf: None
        }
    }

    pub fn range(&mut self, range: Range) -> &mut Self {
        self.range = Some(range);
        self
    }

    pub fn odr(&mut self, odr: ODR_LPF) -> &mut Self {
        self.odr = Some(odr);
        self
    }

    pub fn hpf(&mut self, hpf: HPF_CORNER) -> &mut Self {
        self.hpf = Some(hpf);
        self
    }
}
