# ADXL355 embedded-hal driver crate



## Usage

Include [library](https://crates.io/crates/adxl355) as a dependency in your Cargo.toml


```
[dependencies.adxl355]
version = "<version>"
```

Use embedded-hal implementation to get SPI and a GPIO OutputPin for the chip select, then create the accelerometer handle

```rust

use adxl355::{Adxl355, Accelerometer};

// to create sensor with default configuration:
let mut accelerometer = Adxl355::default(spi, cs)?;

// start measurements
accelerometer.start();

// to get 3d accerlation data:
let accel = accelerometer.acceleration()?;
println!("{:?}", accel);


// One can also use conf module to supply configuration:

use adxl355::{Adxl355, Config as ADXLConfig, ODR_LPF, Range, Accelerometer};

Adxl355::new(spi, cs,
                     ADXLConfig::new()
                     .odr(ODR_LPF::ODR_31_25_Hz)
                     .range(Range::_2G))?;
```