use dht_lib::{Sensor, read};
use std::time::Duration;

fn main() {
    loop {
        match read(Sensor::Dht11, 14) {
            Ok(r) => {
                println!("Read data: Temperature {}C Humidity {}%", r.temperature, r.humidity);
            }
            Err(e) => {
                eprintln!("Could not read data: {:?}", e);
            }
        }
        std::thread::sleep(Duration::from_secs(4));
    }
}
