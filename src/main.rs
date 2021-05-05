mod sensors;

use dht_lib::read;
use sensors::DhtSensor;
use std::time::Duration;

const TEST_PIN: u8 = 4;

fn main() {
    let conf = sensors::SensorConfig::build(vec![
        sensors::Sensor::dht_11(TEST_PIN),
    ], 1);
    let s = serde_yaml::to_string(&conf).unwrap();
    for line in s.lines() {
        println!("{}", line);
    }

    loop {
        let now = std::time::Instant::now();
        for sensor in conf.sensors() {
            match sensor.description() {
                Some(s) => println!("Reading sensor {}", s),
                None => println!("Reading sensor on pin {}", sensor.pin())
            }
            let reading = sensor.read();
            match reading {
                Err(e) => eprintln!("Reading error: {:#?}", e),
                Ok(o) => println!("Reading: t: {} h: {}", o.temperature, o.humidity),
            }
        }
        // TODO don't unwrap
        let elapsed = now.elapsed();
        let to_subtract = Duration::from_secs(conf.min_read_time()).checked_sub(elapsed);

        let to_wait = match to_subtract {
            None => Some(Duration::from_secs(conf.read_interval())),
            Some(t) => Duration::from_secs(conf.read_interval()).checked_sub(t),
        };
        match to_wait {
            Some(t) => std::thread::sleep(t),
            _ => {}
        }
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
