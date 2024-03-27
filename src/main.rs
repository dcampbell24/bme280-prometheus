use std::fmt::Write;

use bme280::{i2c::BME280, Measurements};
use embedded_hal::delay::DelayNs;
use linux_embedded_hal::{Delay, I2CError, I2cdev};
use metrics::gauge;
use metrics_exporter_prometheus::PrometheusBuilder;
// use rand::Rng;

// use std::{thread, time};

fn main() {
    let builder = PrometheusBuilder::new();
    // Defaults to enabled, listening at 0.0.0.0:9000
    builder
        .install()
        .expect("failed to install recorder/exporter");

    let humidity = gauge!("humidity %");
    let pressure = gauge!("pressure pascals");
    let temperature_c = gauge!("temperature celsius ");
    let temperature_f = gauge!("temperature fahrenheit");

    // Using Linux I2C Bus #1 in this example.
    let i2c_bus = I2cdev::new("/dev/i2c-1").unwrap();
    // Initialize the BME280 using the primary I2C address 0x76.
    let mut bme280 = BME280::new_primary(i2c_bus);

    let mut delay = Delay;
    bme280.init(&mut delay).unwrap();

    // let mut rng = rand::thread_rng();
    // let one_second = time::Duration::from_millis(1_000);
    loop {
        let measurements = bme280.measure(&mut delay).unwrap();
        // let r: f64 = rng.gen();

        let str = write_measurements(&measurements);
        print!("{}{}{}", termion::clear::All, termion::cursor::Goto(0, 0), str);

        humidity.set(measurements.humidity);
        pressure.set(measurements.pressure);
        temperature_c.set(measurements.temperature);
        temperature_f.set((measurements.temperature * 1.8) + 32.0);
        // temperature.set(r);

        delay.delay_ms(60_000);
        // thread::sleep(one_second);
    }
}

fn write_measurements(measurements: &Measurements<I2CError>) -> String {
    // Change to round_ties_even when stabilized.
    let rh = (measurements.humidity * 10.0).round() / 10.0;
    let t1 = (measurements.temperature * 10.0).round() / 10.0;
    let mut t2 = (measurements.temperature * 1.8) + 32.0;
    t2 = (t2 * 100.0).round() / 100.0;

    let mut out = String::new();
    writeln!(out, "Pressure: {} ± 100 pascals", measurements.pressure.round()).unwrap();
    writeln!(out, "Relative Humidity: {rh:.1} ± 3 %").unwrap();
    writeln!(out, "Temperature: {t1:.1} ± 1 °C  {t2:.2} ± 1.8 °F").unwrap();
    out
}
