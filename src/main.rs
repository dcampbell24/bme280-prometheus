use std::fmt;

use bme280::{i2c::BME280, Measurements};
use embedded_hal::delay::DelayNs;
use linux_embedded_hal::{Delay, I2CError, I2cdev};
use metrics::gauge;
use metrics_exporter_prometheus::PrometheusBuilder;

fn main() {
    let builder = PrometheusBuilder::new();
    // Defaults to enabled, listening at 0.0.0.0:9000
    builder
        .install()
        .expect("failed to install recorder/exporter");

    let humidity = gauge!("humidity_percent");
    let pressure = gauge!("pressure_pascals");
    let temperature_c = gauge!("temperature_celsius");
    let temperature_f = gauge!("temperature_fahrenheit");

    let i2c_bus = I2cdev::new("/dev/i2c-1").unwrap();
    // Initialize the BME280 using the primary I2C address 0x76.
    let mut bme280 = BME280::new_primary(i2c_bus);

    let mut delay = Delay;
    bme280.init(&mut delay).unwrap();

    loop {
        let measurements = bme280.measure(&mut delay).unwrap();
        let measurements = remove_error(measurements);

        // print!("{}{}{}", termion::clear::All, termion::cursor::Goto(0, 0), measurements);

        humidity.set(measurements.humidity);
        pressure.set(measurements.pressure);
        temperature_c.set(measurements.temperature_c);
        temperature_f.set(measurements.temperature_f);

        delay.delay_ms(1_000);
    }
}

struct MeasurementsAdjusted {
    humidity: f32,
    pressure: f32,
    temperature_c: f32,
    temperature_f: f32,
}

fn remove_error(measurements: Measurements<I2CError>) -> MeasurementsAdjusted {
    // Change to round_ties_even when stabilized.
    let humidity = (measurements.humidity * 10.0).round() / 10.0;
    let pressure = measurements.pressure.round();
    let temperature_c = (measurements.temperature * 10.0).round() / 10.0;
    let mut temperature_f = (measurements.temperature * 1.8) + 32.0;
    temperature_f = (temperature_f * 100.0).round() / 100.0;
    MeasurementsAdjusted {
        humidity,
        pressure,
        temperature_c,
        temperature_f,
    }
}

impl fmt::Display for MeasurementsAdjusted {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Pressure: {} ± 100 pascals", self.pressure)?;
        writeln!(f, "Relative Humidity: {:.1} ± 3 %", self.humidity)?;
        writeln!(
            f,
            "Temperature: {:.1} ± 1 °C  {:.2} ± 1.8 °F",
            self.temperature_c, self.temperature_f
        )
    }
}
