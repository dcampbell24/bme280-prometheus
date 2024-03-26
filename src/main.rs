use std::fmt::{self, Write};

use bme280::{i2c::BME280, Measurements};
use embedded_hal::delay::DelayNs;
use linux_embedded_hal::{Delay, I2CError, I2cdev};

fn main() {
    let mut bme280 = BME280Prometheus::init();
    loop {
        bme280.read();
        print!(
            "{}{}{}",
            termion::clear::All,
            termion::cursor::Goto(1, 1),
            bme280
        );
        bme280.wait_ms(1_000);
    }
}

struct BME280Prometheus {
    bme280: BME280<I2cdev>,
    delay: Delay,
    measurements: Option<Result<Measurements<I2CError>, bme280::Error<I2CError>>>,
}

impl BME280Prometheus {
    fn init() -> Self {
        // Using Linux I2C Bus #1 in this example.
        let i2c_bus = I2cdev::new("/dev/i2c-1").unwrap();
        let mut delay = Delay;

        // Initialize the BME280 using the primary I2C address 0x76.
        let mut bme280 = BME280::new_primary(i2c_bus);
        bme280.init(&mut delay).unwrap();

        BME280Prometheus {
            bme280,
            delay,
            measurements: None,
        }
    }

    fn read(&mut self) {
        self.measurements = Some(self.bme280.measure(&mut self.delay));
    }

    fn wait_ms(&mut self, ms: u32) {
        self.delay.delay_ms(ms);
    }
}

impl fmt::Display for BME280Prometheus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut out = String::new();
        match &self.measurements {
            Some(measurements) => {
                match measurements {
                    // Change to round_ties_even when stabilized.
                    Ok(measurements) => {
                        let rh = (measurements.humidity * 10.0).round() / 10.0;
                        let t1 = (measurements.temperature * 10.0).round() / 10.0;
                        let mut t2 = (measurements.temperature * 1.8) + 32.0;
                        t2 = (t2 * 100.0).round() / 100.0;

                        writeln!(out, "Relative Humidity: {rh:.1} ± 3 %")?;
                        writeln!(out, "Temperature: {t1:.1} ± 1 °C  {t2:.2} ± 1.8 °F")?;
                        writeln!(
                            out,
                            "Pressure: {} ± 100 pascals",
                            measurements.pressure.round()
                        )?;
                    }
                    Err(e) => writeln!(out, "{e:?}")?,
                }
            }
            None => writeln!(out, "no readings taken yet")?,
        }

        write!(f, "{out}")
    }
}
