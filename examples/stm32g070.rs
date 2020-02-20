//! This example is based on the stm32g070 microcontroller and assumes that you're using the
//! [NUCLEO-G070RB] development board by ST. However it should work with any board that has the correct pins available.
//!
//! [NUCLEO-G070RB]: https://www.st.com/en/evaluation-tools/nucleo-g070rb.html
//!
//! The pin connections are:
//!
//! SPI
//! ===
//! sck = PB3;
//! miso = PB4;
//! mosi = PB5;
//! cs = PB0
//!
//! USART2 @ 1000000 Baud
//! ====
//! Tx = PA2
//!
#![no_std]
#![no_main]

use core::fmt::Write;

// pick a panicking behavior
extern crate panic_halt; // you can put a breakpoint on `rust_begin_unwind` to catch panics

use cortex_m_rt::entry;

use embedded_hal as hal;
use hal::digital::v2::OutputPin;


use stm32g0xx_hal::{
    prelude::*,
    stm32,
    spi,
    serial::Config,
    gpio,
    exti::Event
};


use adxl355::{Adxl355, Config as ADXLConfig, ODR_LPF, Range, RawAccelerometer};

#[entry]
fn main() -> ! {
    let dp = stm32::Peripherals::take().expect("cannot take peripherals");
    let mut rcc = dp.RCC.constrain();
    let gpioa = dp.GPIOA.split(&mut rcc);
    let gpiob = dp.GPIOB.split(&mut rcc);

    let tx = gpioa.pa2;
    let rx = gpioa.pa3;
    let mut usart = dp
        .USART2
        .usart(tx, rx, Config::default().baudrate(1000000.bps()), &mut rcc)
        .unwrap();

    writeln!(usart, "Hello stm32g0\n").unwrap();

    let mut led = gpioa.pa5.into_push_pull_output();
    let drdy = gpioa.pa0.into_pull_down_input();

    let mut exti = dp.EXTI;
    drdy.listen(gpio::SignalEdge::Rising, &mut exti);

    let mut cs = gpiob.pb0.into_push_pull_output();
    cs.set_high().unwrap();

    let sck = gpiob.pb3;
    let miso = gpiob.pb4;
    let mosi = gpiob.pb5;
    let spi = dp.SPI1.spi(
        (sck, miso, mosi),
        spi::MODE_0,
        8000.khz(),
        &mut rcc);

    let mut cfg = ADXLConfig::new();
    cfg.odr(ODR_LPF::ODR_3_906_Hz)
       .range(Range::_4G);

    let mut accelerometer = Adxl355::new(spi, cs, &cfg).unwrap();
    let id = accelerometer.get_device_id();

    writeln!(usart, "Got device ID {}", id).unwrap();
    if id == 0xED {
        led.set_high().unwrap();
    }

    accelerometer.start();

    writeln!(usart, "Start!").unwrap();

    let temp: u16 = accelerometer.read_temp_raw();

    writeln!(usart, "Temp: {}", temp).unwrap();

    loop {

        if exti.is_pending(Event::GPIO0, gpio::SignalEdge::Rising) {
            led.set_high().unwrap();
            exti.unpend(Event::GPIO0);

            let accel = accelerometer.accel_raw().unwrap();

            writeln!(usart, "{},{},{}", accel.x, accel.y, accel.z).unwrap();
            led.set_low().unwrap();
        }



    }
}
