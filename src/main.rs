mod sensors;

use dht_lib::read;
use sensors::DhtSensor;
use std::time::Duration;
use std::path::PathBuf;
use std::fs::OpenOptions;
use std::io::Read;

const TEST_PIN1: u8 = 4;
const TEST_PIN2: u8 = 17;
const TEST_PIN3: u8 = 27;

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
        sensors::Sensor::named_dht_22(TEST_PIN1, "TEST --- A"),
        sensors::Sensor::named_dht_22(TEST_PIN2, "TEST --- B"),
        sensors::Sensor::named_dht_22(TEST_PIN3, "TEST --- C"),
    ], 1);
    let s = serde_yaml::to_string(&conf).unwrap();
    for line in s.lines() {
        println!("{}", line);
    }

    // std::process::Command::new("raspistill").args([
    //     "-o",
    //     "~/Desktop/test2.jpg"
    // ].iter()).output().unwrap();
    // return;

    loop {
        let now = std::time::Instant::now();
        for sensor in conf.sensors() {
            match sensor.description() {
                Some(s) => bunt::println!("Reading sensor {[yellow]}", s),
                None => bunt::println!("Reading sensor on pin {[yellow]}", sensor.pin())
            }
            let reading = sensor.read();
            match reading {
                Err(e) => bunt::eprintln!("{$red}Reading error{/$}: {:#?}", e),
                Ok(o) => bunt::println!("Reading: t: {[green]} h: {[green]}", o.temperature, o.humidity),
            }
        }
        let to_add = Duration::from_secs(conf.min_read_time()).checked_sub(now.elapsed());

        let to_wait = match to_add {
            None => Some(Duration::from_secs(conf.read_interval())),
            Some(t) => Duration::from_secs(conf.read_interval()).checked_add(t),
        };
        match to_wait {
            Some(t) => {
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
