use linux_embedded_hal::{Delay, I2cdev};
use bme280::i2c::BME280;

use embedded_hal::delay::DelayNs;

fn main () {
    // using Linux I2C Bus #1 in this example
    let i2c_bus = I2cdev::new("/dev/i2c-1").unwrap();
    let mut delay = Delay;

    // initialize the BME280 using the primary I2C address 0x76
    let mut bme280 = BME280::new_primary(i2c_bus);

    delay.delay_ms(1_000);

    // initialize the sensor
    bme280.init(&mut delay).unwrap();

    // measure temperature, pressure, and humidity
    // Change to round_ties_even when stabalized.
    let measurements = bme280.measure(&mut delay).unwrap();
    let rh = (measurements.humidity * 10.0).round() / 10.0;
    let t1 = (measurements.temperature * 10.0).round() / 10.0;
    let mut t2 = (measurements.temperature * 1.8) + 32.0;
    t2 = (t2 * 100.0).round() / 100.0;

    println!("Relative Humidity: {rh:.1} ± 3 %");
    println!("Temperature: {t1:.1} ± 1 °C  {t2:.2} ± 1.8 °F");
    println!("Pressure: {} ± 100 pascals", measurements.pressure.round());
}
