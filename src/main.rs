mod sensors;

use dht_lib::read;
use sensors::DhtSensor;
use std::time::Duration;
use std::path::PathBuf;
use std::fs::{OpenOptions, File};
use std::io::Read;
use std::str::FromStr;

use chrono::prelude::*;

use eye::prelude::*;

fn main() {
    let path: PathBuf = "/home/pi/Dokumente/thermometer-config.yaml".into();
    let mut file = OpenOptions::new().read(true).open(path).expect("Could not open config file");
    let mut config_str = String::new();
    file.read_to_string(&mut config_str).expect("Could not read config file to string");

    let conf: sensors::SensorConfig = serde_yaml::from_str(config_str.as_str())
        .expect("Could not deserialize config");

    let output_path = PathBuf::from_str(conf.output_path()).unwrap();
    if !output_path.exists() {
        bunt::println!("{$blue}Creating output directory {[green]}{/$}", output_path.to_str().unwrap());
        std::fs::create_dir_all(&output_path);
    }

    bunt::println!("{$blue}Reading camera data{/$}");
    let cam_info = rascam::info().expect("Couldn't read camera data!");
    let has_cam = if cam_info.cameras.len() > 0 {
        bunt::println!("There are {[green]} cameras connected", cam_info.cameras.len());
        true
    } else {
        bunt::println!("{$yellow}No camera detected!{/$}");
        false
    };
    println!("-----------------------------");
    println!("rascam cam data:");
    for cam in cam_info.cameras {
        println!(
            "Camera: {}\n- max height: {}\n- max width: {}\n- port: {}\n- has lens: {}",
            cam.camera_name,
            cam.max_height,
            cam.max_width,
            cam.port_id,
            cam.lens_present
        )
    }

    println!("-----------------------------");
    println!("eye-rs cam data:");
    let ctx = Context::new();

    let devices = ctx.query_devices().expect("eye-rs couldn't query context");
    if devices.len() > 0 {
        for cam in devices {
            println!("Cam: {}", cam);
            let test_path: PathBuf = "/home/pi/Desktop/test".into();
            std::fs::create_dir(test_path);
            match Device::with_uri(&cam) {
                Ok(dev) => {
                    if let Ok(controls) = dev.query_controls() {
                        for control in controls {
                            println!("Control of {}", cam);
                            println!(
                                "- name: {}\n- flags: {:?}\n- type: {:?}\n- id: {}",
                                control.name,
                                control.flags,
                                control.typ,
                                control.id
                            );
                        }
                    }
                    if let Ok(streams) = dev.query_streams() {
                        for control in streams {
                            println!("Stream of {}", cam);
                            println!(
                                "- height: {}\n- width: {}\n- flags: {:?}\n- interval: {:?}\n- pixfmt: {:?}",
                                control.height,
                                control.width,
                                control.flags,
                                control.interval,
                                control.pixfmt
                            );
                        }
                    }
                },
                Err(e) => println!("Couldn't get device {}. Error: {}", cam, e)
            }

        }
    } else {
        println!("eye-rs couldn't find any cameras");
    }

    let mut last_reading_time = Local::now();
    let mut data_output_path = output_path.clone();
    data_output_path.push("data");
    std::fs::create_dir_all(&data_output_path);
    data_output_path.push(format!("readings-{}.csv", last_reading_time.to_string()));
    let mut csv = csv::WriterBuilder::new()
        .delimiter(conf.delimiter())
        .from_path(&data_output_path).unwrap();

    let mut img_output_path = output_path.clone();
    img_output_path.push("img");
    let img_output_path = img_output_path;

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
            data_output_path.set_file_name(format!("readings-{}.csv", last_reading_time.to_string()));
            csv = csv::WriterBuilder::new()
                .delimiter(conf.delimiter()) // TODO
                .from_path(&data_output_path).unwrap();
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
        // TODO take image
        // TODO somehow retry sensors that failed
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
