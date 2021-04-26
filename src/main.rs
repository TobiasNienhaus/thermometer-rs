mod sensors;

use dht_lib::read;
use sensors::DhtSensor;
use std::time::Duration;

const PIN: u8 = 4;

fn main() {
    let s = sensors::SensorCollection::test();
    let s = serde_yaml::to_string(&s).unwrap();
    for line in s.lines() {
        println!("{}", line);
    }
    loop {
        let mut reading = read(DhtSensor::Dht11, PIN);
        while let Err(e) = &reading {
            eprintln!("Could not read data: {:?}", e);
            reading = read(DhtSensor::Dht11, PIN);
        }
        println!("Read data: Temperature {}C Humidity {}%", reading.temperature, reading.humidity);
        std::thread::sleep(Duration::from_secs(4));
    }
}
