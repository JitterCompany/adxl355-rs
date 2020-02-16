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

impl From<Range> for f32 {
    fn from(range: Range) -> f32 {
        match range {
            Range::_2G => 2.048,
            Range::_4G => 4.096,
            Range::_8G => 8.192,
        }
    }
}

impl Default for Range {
    fn default() -> Self {
        Range::_2G
    }
}

#[derive(Copy, Clone, Debug)]
/// Output data rate (odr) and Low pass filter corner frequency (lpf)
pub enum ODR_LPF {
    /// odr = 4000 Hz and lpf = 1000 Hz
    ODR_4000_Hz                = 0,
    /// odr = 2000 Hz and lpf = 500 Hz
    ODR_2000_Hz                = 1,
    /// odr = 1000 Hz and lpf = 250 Hz
    ODR_1000_Hz                = 2,
    /// odr = 500 Hz and lpf = 125 Hz
    ODR_500_Hz                 = 3,
    /// odr = 250 Hz and lpf = 62.5 Hz
    ODR_250_Hz                 = 4,
    /// odr = 125 Hz and lpf = 31.25 Hz
    ODR_125_Hz                 = 5,
    /// odr = 62.5 Hz and lpf = 15.625 Hz
    ODR_62_5_Hz                = 6,
    /// odr = 31.25 Hz and lpf = 7.813 Hz
    ODR_31_25_Hz               = 7,
    /// odr = 15.625 Hz and lpf = 3.906
    ODR_15_625_Hz              = 8,
    /// odr = 7.813 Hz and lpf = 1.953 Hz
    ODR_7_813_Hz               = 9,
    /// odr = 3.906 Hz and lpf = 0.977 Hz
    ODR_3_906_Hz               = 10,
}

impl ODR_LPF {
    pub fn val(self) -> u8 {
        self as u8
    }
}

impl From<ODR_LPF> for f32 {
    fn from(rate: ODR_LPF) -> f32 {
        match rate {
            ODR_LPF::ODR_4000_Hz => 4000.0,
            ODR_LPF::ODR_2000_Hz => 2000.0,
            ODR_LPF::ODR_1000_Hz => 1000.0,
            ODR_LPF::ODR_500_Hz => 500.0,
            ODR_LPF::ODR_250_Hz => 250.0,
            ODR_LPF::ODR_125_Hz => 125.0,
            ODR_LPF::ODR_62_5_Hz => 62.5,
            ODR_LPF::ODR_31_25_Hz => 31.25,
            ODR_LPF::ODR_15_625_Hz => 15.625,
            ODR_LPF::ODR_7_813_Hz => 7.813,
            ODR_LPF::ODR_3_906_Hz => 3.906,
        }
    }
}

impl Default for ODR_LPF {
    fn default() -> Self {
        ODR_LPF::ODR_3_906_Hz
    }
}

#[derive(Copy, Clone, Debug)]
/// High pass corner frequency is proportional to the output data rate (ODR)
pub enum HPF_CORNER {
    /// no high pass filter
    NONE        = 0,
    /// corner freq = 247 × 10^3 × ODR
    _247_ODR    = 1,
    /// corner freq = 62.048 × 10^3 × ODR
    _62_084_ODR = 2,
    /// corner freq = 15.454 × 10^3 × ODR
    _15_545_ODR = 3,
    /// corner freq = 3.862 × 10^3 × ODR
    _3_862_ODR  = 4,
    /// corner freq = 0.954 × 10^3 × ODR
    _0_954_ODR  = 5,
    /// corner freq = 0.238 × 10^3 × ODR
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

/// ADXL355 configuration struct
impl Config {

    // Creates a new configuration object with default values
    pub fn new() -> Self {
        Config {
            range: None,
            odr: None,
            hpf: None
        }
    }

    /// Sets the range configuration
    /// Default is 2G
    pub fn range(&mut self, range: Range) -> &mut Self {
        self.range = Some(range);
        self
    }

    /// Sets the output data rate and low pass filter settings.
    /// Default data rate is `3.906 Hz`
    /// The low pass filter is fixed as 1/4 of the output data rate (fs)
    pub fn odr(&mut self, odr: ODR_LPF) -> &mut Self {
        self.odr = Some(odr);
        self
    }

    /// Sets the -3dB corner frequency for the high pass filter
    ///
    /// Default is no high pass filter
    pub fn hpf(&mut self, hpf: HPF_CORNER) -> &mut Self {
        self.hpf = Some(hpf);
        self
    }
}
