mod sensors;

use dht_lib::read;
use sensors::DhtSensor;
use std::time::Duration;

const PIN: u8 = 4;

fn main() {
    let s = sensors::SensorConfig::build(vec![
        sensors::Sensor::dht_11(0),
        sensors::Sensor::dht_11(1),
        sensors::Sensor::dht_11(2),
        sensors::Sensor::dht_11(3),
        sensors::Sensor::dht_22(4),
        sensors::Sensor::dht_22(5),
        sensors::Sensor::dht_22(6),
        sensors::Sensor::dht_22(7),
    ], 10);
    let s = serde_yaml::to_string(&s).unwrap();
    for line in s.lines() {
        println!("{}", line);
    }
    // loop {
    //     let mut reading = read(DhtSensor::Dht11, PIN);
    //     while let Err(e) = &reading {
    //         eprintln!("Could not read data: {:?}", e);
    //         reading = read(DhtSensor::Dht11, PIN);
    //     }
    //     let reading = reading.unwrap();
    //     println!("Read data: Temperature {}C Humidity {}%", reading.temperature, reading.humidity);
    //     std::thread::sleep(Duration::from_secs(4));
    // }
}
