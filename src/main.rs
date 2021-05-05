mod sensors;

use dht_lib::read;
use sensors::DhtSensor;
use std::time::Duration;
use std::path::PathBuf;
use std::fs::OpenOptions;
use std::io::Read;

const TEST_PIN: u8 = 4;

fn main() {
    // let path: PathBuf = "~/Dokumente/thermometer-config.yaml".into();
    // let mut file = OpenOptions::new().read(true).open(path).expect("Could not open config file");
    // let mut config_str = String::new();
    // file.read_to_string(&mut config).expect("Could not read config file to string");
    //
    // let read_conf: sensors::SensorConfig = serde_yaml::from_str(config_str.as_str())
    //     .expect("Could not deserialize config");

    // let conf = read_conf;
    let conf = sensors::SensorConfig::build(vec![
        sensors::Sensor::named_dht_11(TEST_PIN, "TEST"),
    ], 1);
    let s = serde_yaml::to_string(&conf).unwrap();
    for line in s.lines() {
        println!("{}", line);
    }
    return;

    loop {
        let now = std::time::Instant::now();
        for sensor in conf.sensors() {
            match sensor.description() {
                Some(s) => println!("Reading sensor {}", s),
                None => println!("Reading sensor on pin {}", sensor.pin())
            }
            let reading = sensor.read();
            match reading {
                Err(e) => bunt::eprintln!("{$red}Reading error{/$}: {:#?}", e),
                Ok(o) => bunt::println!("Reading: t: {[green]} h: {[green]}", o.temperature, o.humidity),
            }
        }
        // TODO don't unwrap
        let elapsed = now.elapsed();
        println!("Elapsed {} ms", elapsed.as_millis());
        let to_add = Duration::from_secs(conf.min_read_time()).checked_sub(elapsed);
        println!("To subtract: {:?} ms", to_add.map(|s| s.as_millis()));

        let to_wait = match to_add {
            None => Some(Duration::from_secs(conf.read_interval())),
            Some(t) => Duration::from_secs(conf.read_interval()).checked_add(t),
        };
        println!("To wait: {:?} ms", to_wait.map(|s| s.as_millis()));
        match to_wait {
            Some(t) => {
                println!("Sleeping {} ms", t.as_millis());
                std::thread::sleep(t)
            },
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
