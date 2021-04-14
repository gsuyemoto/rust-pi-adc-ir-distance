extern crate rust_gpiozero;
extern crate raspi_adc_ir;

use crate::raspi_adc_ir::ir_distance::IRDistance;
use rust_gpiozero::*;

use std::time::Duration;
use std::thread;

fn main() {
    env_logger::init();

    let mut led = LED::new(21); 
    led.set_blink_count(5);
    led.blink(0.5, 0.5);

    println!("Starting loop...");

    loop {
        //let distance = IRDistance::get_distance(None);
        let distance = IRDistance::get_distance(None);
        println!("reading: {}", distance);
        
        if distance < 130.0 {
            led.on();
        }
        else {
            led.off();
        }
        
        thread::sleep(Duration::from_millis(50));
    }
} 
