//! This example is based on the stm32f103 blackpill board
//! However it should work with any board that has the correct pins available.
//!
//!
//! The pin connections are:
//!
//! SPI
//! ===
//! sck = PA5;
//! miso = PA6;
//! mosi = PA7;
//! cs = PA1
//!
//! data_ready = PA0
//!
//! USART1 @ 1000000 Baud
//! ====
//! Tx = PA9
//!

#![deny(unsafe_code)]
#![no_std]
#![no_main]

use panic_halt as _;

use stm32f1xx_hal::{
    prelude::*,
    pac,
    serial::{Serial, Config},
    spi::{Spi, Mode, Polarity, Phase},
    gpio::{ExtiPin, Edge}
};
use cortex_m_rt::entry;
use embedded_hal::digital::v2::OutputPin;

use adxl355::{Adxl355, Config as ADXLConfig, ODR_LPF, Range, RawAccelerometer, Accelerometer};
use core::fmt::Write;

const BAUDRATE: u32 = 1_000_000;


#[entry]
fn main() -> ! {
    // Get access to the core peripherals from the cortex-m crate
    let _cp = cortex_m::Peripherals::take().unwrap();
    // Get access to the device specific peripherals from the peripheral access crate
    let dp = pac::Peripherals::take().unwrap();

    // Take ownership over the raw flash and rcc devices and convert them into the corresponding
    // HAL structs
    let mut flash = dp.FLASH.constrain();
    let mut rcc = dp.RCC.constrain();

    // Freeze the configuration of all the clocks in the system and store the frozen frequencies in
    // `clocks`
    // let clocks = rcc.cfgr.freeze(&mut flash.acr);
    let clocks = rcc.cfgr.use_hse(8.mhz()).sysclk(72.mhz()).pclk1(36.mhz()).freeze(&mut flash.acr);

    // Acquire the GPIOC peripheral
    let mut gpiob = dp.GPIOB.split(&mut rcc.apb2);
    let mut gpioa = dp.GPIOA.split(&mut rcc.apb2);

    // Configure gpio B pin 12 as a push-pull output. The `crh` register is passed to the function
    // in order to configure the port. For pins 0-7, crl should be passed instead.
    let mut led = gpiob.pb12.into_push_pull_output(&mut gpiob.crh);

    let mut afio = dp.AFIO.constrain(&mut rcc.apb2);

    let tx = gpioa.pa9.into_alternate_push_pull(&mut gpioa.crh);
    let rx = gpioa.pa10;

    let serial = Serial::usart1(
        dp.USART1,
        (tx, rx),
        &mut afio.mapr,
        Config::default().baudrate(BAUDRATE.bps()),
        clocks,
        &mut rcc.apb2,
    );
    let (mut tx, _) = serial.split();

    let mut drdy = gpioa.pa0.into_floating_input(&mut gpioa.crl);
    let mut cs = gpioa.pa1.into_push_pull_output(&mut gpioa.crl);


    let sck = gpioa.pa5.into_alternate_push_pull(&mut gpioa.crl);
    let miso = gpioa.pa6.into_floating_input(&mut gpioa.crl);
    let mosi = gpioa.pa7.into_alternate_push_pull(&mut gpioa.crl);

    let spi_mode = Mode {
        polarity: Polarity::IdleLow,
        phase: Phase::CaptureOnFirstTransition
    };

    let spi = Spi::spi1(
        dp.SPI1,
        (sck, miso, mosi),
        &mut afio.mapr,
        spi_mode,
        8000.khz(),
        clocks,
        &mut rcc.apb2);

    let mut cfg = ADXLConfig::new();
    cfg.odr(ODR_LPF::ODR_3_906_Hz)
        .range(Range::_4G);

    cs.set_high().unwrap();
    let mut accelerometer = Adxl355::new(spi, cs, &cfg).unwrap();
    let id = accelerometer.get_device_id();

    writeln!(tx, "Got device ID {}", id).unwrap();
    if id == 0xED {
        led.set_high().unwrap();
        writeln!(tx, "Correct!").unwrap();
    }

    drdy.make_interrupt_source(&mut afio);
    drdy.trigger_on_edge(&dp.EXTI, Edge::RISING);
    drdy.enable_interrupt(&dp.EXTI);

    accelerometer.start();

    loop {

        if drdy.check_interrupt() {
            led.toggle().unwrap();

            let accel = accelerometer.accel_raw().unwrap();
            let norm = accelerometer.accel_norm().unwrap();
            writeln!(tx, "raw x={:.1}, y={:.1}, z={:.1}\nnormalized [g]: x={:.3}, y={:.3}, z={:.3}", accel.x, accel.y, accel.z, norm.x, norm.y, norm.z).unwrap();

            // if we don't clear this bit, the ISR would trigger indefinitely
            drdy.clear_interrupt_pending_bit();
        }
    }
}
