pub mod ir_distance {
    use std::thread;
    use std::time::Duration;
    
    use linux_embedded_hal::I2cdev;
    use ads1x1x::{Ads1x1x, ModeChangeError, SlaveAddr};
    
    const MEDIAN_READINGS: usize    = 11;
    const MEDIAN_INDEX: usize       = 6;
    const DEVICE_PATH: &str         = "/dev/i2c-1";
    
    pub struct IRDistance {
        reading:    i16,
    }

    impl IRDistance {
        pub fn new() -> Self {
            IRDistance { reading: 0 }
        }

        pub fn get_median_reading(&mut self) -> i16 {
            let mut last_ten    = Vec::with_capacity(MEDIAN_READINGS);
            let dev             = I2cdev::new(DEVICE_PATH).unwrap();
            let address         = SlaveAddr::default();
            let adc             = Ads1x1x::new_ads1115(dev, address);

            // change mode to continuous readings
            match adc.into_continuous() {
                Err(ModeChangeError::I2C(e, adc)) => panic!("Failed to change mode: {}", e),
                Ok(mut adc) => {
                    for _ in 0..MEDIAN_READINGS {
                        let distance = adc.read().unwrap();
                    
                        last_ten.push(distance); 
                        thread::sleep(Duration::from_millis(5));
                    }

                    self.reading = *last_ten.select_nth_unstable(MEDIAN_INDEX).1;
                    println!("reading: {}", self.reading);
                    self.reading
                },
            }
    
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    pub trait DetectPerson {
        fn person_detected(&mut self, min_distance: i16) -> bool;
    }
    
    impl DetectPerson for ir_distance::IRDistance {
        fn person_detected(&mut self, min_distance: i16) -> bool {
            self.get_median_reading() < min_distance
        }
    }
    
    #[test]
    fn test1() {
        let mut ultrasonic = ir_distance::IRDistance::new();
        assert_eq!(false, ultrasonic.person_detected(30));
    }
}
