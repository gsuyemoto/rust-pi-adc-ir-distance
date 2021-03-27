use std::thread;
use std::time::Duration;

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
    
                if measurement < 8000 {
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
