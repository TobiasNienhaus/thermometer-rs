mod sensors;

use dht_lib::read;
use sensors::DhtSensor;
use std::time::Duration;
use std::path::PathBuf;
use std::fs::{OpenOptions, File};
use std::io::Read;
use std::str::FromStr;

use chrono::prelude::*;

fn main() {
    let path: PathBuf = "/home/pi/Dokumente/thermometer-config.yaml".into();
    let mut file = OpenOptions::new().read(true).open(path).expect("Could not open config file");
    let mut config_str = String::new();
    file.read_to_string(&mut config_str).expect("Could not read config file to string");

    let conf: sensors::SensorConfig = serde_yaml::from_str(config_str.as_str())
        .expect("Could not deserialize config");

    let mut output_path = PathBuf::from_str(conf.output_path()).unwrap();
    if !output_path.exists() {
        bunt::println!("{$blue}Creating output directory {[green]}{/$}", output_path.to_str().unwrap());
        std::fs::create_dir_all(&output_path);
    }

    let mut last_reading_time = Local::now();
    output_path.push(format!("readings-{}.csv", last_reading_time.to_string()));
    let mut csv = csv::WriterBuilder::new()
        .delimiter(conf.delimiter()) // TODO
        .from_path(&output_path).unwrap();

    let mut headers = vec!["date".to_owned()];
    for (idx, s) in conf.sensors().iter().enumerate() {
        let s = s.description()
            .map(|s| s.to_owned())
            .unwrap_or(idx.to_string());
        let a = format!("{} - Temperatur", s);
        let b = format!("{} - Luftfeuchtigkeit", s);
        headers.push(a);
        headers.push(b);
    }

    csv.write_record(&headers).unwrap();
    csv.flush().unwrap();

    loop {
        let now = Local::now();
        if last_reading_time.day() != now.day() {
            last_reading_time = now;
            output_path.set_file_name(format!("readings-{}.csv", last_reading_time.to_string()));
            csv = csv::Writer::from_path(&output_path).unwrap();
            csv.write_record(&headers).unwrap();
        } else {
            last_reading_time = now;
        }

        let mut readings = Vec::with_capacity(conf.sensors().len());

        let now = std::time::Instant::now();
        for sensor in conf.sensors() {
            match sensor.description() {
                Some(s) => bunt::println!("Reading sensor {[yellow]}", s),
                None => bunt::println!("Reading sensor on pin {[yellow]}", sensor.pin())
            }
            let reading = sensor.read();
            match reading {
                Err(e) => {
                    bunt::eprintln!("{$red}Reading error{/$}: {:#?}", e);
                    readings.push(format!("Error: {:#?}", e));
                    readings.push(format!("Error: {:#?}", e));
                },
                Ok(o) => {
                    bunt::println!("Reading: t: {[green]} h: {[green]}", o.temperature, o.humidity);
                    readings.push(o.temperature.to_string());
                    readings.push(o.humidity.to_string());
                },
            }
        }
        let mut record = vec![last_reading_time.to_string()];
        for r in readings {
            record.push(r);
        }
        csv.write_record(&record).unwrap();
        csv.flush().unwrap();

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
}
