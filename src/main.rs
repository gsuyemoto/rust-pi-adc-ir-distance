use std::thread;
use std::time::Duration;

/*
extern crate embedded_hal;
use embedded_hal::adc::OneShot;

extern crate linux_embedded_hal;

#[macro_use(block)]
extern crate nb;
extern crate ads1x1x;

use ads1x1x::{channel, Ads1x1x, SlaveAddr};
use linux_embedded_hal::I2cdev;

fn main() {
    let dev = match I2cdev::new("/dev/i2c-1") {
        Ok(d)   => d,
        Err(e)  => panic!("Error setting i2c: {}", e),
    };
    let address = SlaveAddr::default();
    let mut adc = Ads1x1x::new_ads1115(dev, address);

    for _ in 0..1000 {
        let reading = match block!(adc.read(&mut channel::SingleA1)) {
            Ok(r)   => r,
            Err(e)  => panic!("Error reading from channel: {:?}", e),
        };

        println!("Channel 1: {}", reading);
        thread::sleep(Duration::from_millis(10));
    }

    let _dev = adc.destroy_ads1115();
}
*/

use linux_embedded_hal::I2cdev;
use ads1x1x::{Ads1x1x, ModeChangeError, SlaveAddr};

fn main() {
    let dev = I2cdev::new("/dev/i2c-1").unwrap();
    let address = SlaveAddr::default();
    let adc = Ads1x1x::new_ads1115(dev, address);
    match adc.into_continuous() {
        Err(ModeChangeError::I2C(e, adc)) => panic!("Failed to change mode: {}", e),
        Ok(mut adc) => {
            loop {
                let measurement = adc.read().unwrap();
                if measurement < 10000 {
                    println!("person detected!");
                }
                else {
                    println!("reading: {}", measurement);
                }
                thread::sleep(Duration::from_millis(10));
            }
        }
    }
}
