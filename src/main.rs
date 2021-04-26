mod sensors;

use dht_lib::read;
use sensors::DhtSensor;
use std::time::Duration;

fn main() {
    let s = sensors::SensorCollection::test();
    let s = serde_yaml::to_string(&s).unwrap();
    println!("{:#?}", s);
    loop {
        match read(DhtSensor::Dht11, 14) {
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
