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

    impl IRDistance {
        pub fn get_raw_reading() -> f32 {
            debug!("Getting raw single reading...");

            let dev             = I2cdev::new(DEVICE_PATH).unwrap();
            let address         = SlaveAddr::default();
            let mut adc         = Ads1x1x::new_ads1115(dev, address);
            let raw_reading     = block!(adc.read(&mut channel::DifferentialA0A1)).unwrap();
            
            raw_reading as f32
        }

        pub fn get_raw_median(num_readings: usize) -> f32 {
            debug!("Getting raw median reading...");

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

            // unable to use the following to find median as the values are float:
            // let median = all_readings.select_nth_unstable(MEDIAN_INDEX).1;
            all_readings.sort_by(|a, b| a.partial_cmp(b).unwrap_or(Equal));
            let median_reading = all_readings[ median ] as f32;

            debug!("Raw median reading: {}", median_reading);
            median_reading
        } 

        pub fn get_distance(num_readings: Option<usize>) -> f32 {
            debug!("Getting distance using model...");

            let num_readings    = num_readings.unwrap_or(10);
            let median          = Self::get_raw_median(num_readings);
            let distance        = Self::dist_with_model(median);

            debug!("Distance with model: {}", distance);
            distance
        }

        // using Y = A + B * ln(X)
        // calculated logarithm regression:
        // Y =  * ln(X)
        // using: https://keisan.casio.com/exec/system/14059930226691
        // using: https://stats.blue/Stats_Suite/logarithmic_regression_calculator.html
        fn dist_with_model(adc_reading: f32) -> f32 {
            debug!("Convert reading to distance...");

            //115.8631_f32 + (-11.2342_f32 * (adc_reading as f32).ln())
            116.4143_f32 + (-11.2966_f32 * (adc_reading as f32).ln())
        }
    }
}
