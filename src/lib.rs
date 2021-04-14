pub mod ir_distance {
    use log::{info, debug, warn};
    use std::cmp::Ordering::Equal;
    use linux_embedded_hal::I2cdev;
    use embedded_hal::adc::OneShot;   
    use nb::block;
    use ads1x1x::{
//                ic::Ads1115,
//                mode::OneShot,
//                ic::Resolution16Bit,
//                interface::I2cInterface,
                channel, Ads1x1x, SlaveAddr,
    };
    const DEVICE_PATH: &str         = "/dev/i2c-1";
    
    pub struct IRDistance;

    impl Drop for IRDistance {
        fn drop(&mut self) {
            
        }
    }

    impl IRDistance {
        pub fn get_raw_reading() -> f32 {
            info!("Getting raw single reading...");

            let dev             = I2cdev::new(DEVICE_PATH).unwrap();
            let address         = SlaveAddr::default();
            let mut adc         = Ads1x1x::new_ads1115(dev, address);
            let raw_reading     = block!(adc.read(&mut channel::DifferentialA0A1)).unwrap();
            
            raw_reading as f32
        }

        pub fn get_raw_median(num_readings: usize) -> f32 {
            info!("Getting raw median reading...");

            let dev             = I2cdev::new(DEVICE_PATH).unwrap();
            let address         = SlaveAddr::default();
            let mut adc         = Ads1x1x::new_ads1115(dev, address);
            let median: usize   = num_readings / 2;
            let mut all_readings: Vec<i16> = Vec::with_capacity(num_readings);

            for _ in 0..num_readings {
                debug!("Entering blocking to get reading from ADC...");

                let reading = block!(adc.read(&mut channel::DifferentialA0A1)).unwrap();
                all_readings.push(reading);

                debug!("Reading to determine median: {}", reading);
            }

            // use the following to find median if the values are float:
            // all_readings.sort_by(|a, b| a.partial_cmp(b).unwrap_or(Equal));
            let median_reading = *all_readings.select_nth_unstable(median).1;
            let median_reading = median_reading as f32;

            info!("Raw median reading: {}", median_reading);
            median_reading
        } 

        pub fn get_distance(num_readings: Option<usize>) -> f32 {
            info!("Getting distance using model...");

            let num_readings    = num_readings.unwrap_or(10);
            let median          = Self::get_raw_median(num_readings);
            let distance        = Self::dist_with_model(median);

            info!("Distance with model: {}", distance);
            distance
        }

        // using Y = A + B * ln(X)
        // calculated logarithm regression:
        // Y =  * ln(X)
        // using: https://keisan.casio.com/exec/system/14059930226691
        // using: https://stats.blue/Stats_Suite/logarithmic_regression_calculator.html
        fn dist_with_model(adc_reading: f32) -> f32 {
            info!("Convert reading to distance...");

            //115.8631_f32 + (-11.2342_f32 * (adc_reading as f32).ln())
            230.867_f32 + (-21.897_f32 * (adc_reading as f32).ln())
        }
    }
}
