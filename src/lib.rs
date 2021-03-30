pub mod ir_distance {
    use std::thread;
    use std::time::Duration;
    
    use linux_embedded_hal::I2cdev;
    use ads1x1x::{Ads1x1x, ModeChangeError, SlaveAddr};
    
    const MEDIAN_READINGS: usize    = 11;
    const MEDIAN_INDEX: usize       = 6;
    const DEVICE_PATH: &str         = "/dev/i2c-1";
    
    pub fn get_median_reading() -> i16 {
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

                let median = *last_ten.select_nth_unstable(MEDIAN_INDEX).1;
                println!("median: {}", median);
                median
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::ir_distance::get_median_reading;

    #[test]
    fn test1() {
        let person_detected = |min_dist| { get_median_reading() < min_dist };
        assert_eq!(false, person_detected(30));
    }
}
