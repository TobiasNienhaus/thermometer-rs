mod sensors;
mod cam;
mod config;

use dht_lib::read;
use sensors::DhtSensor;
use std::time::Duration;
use std::path::PathBuf;
use std::fs::{OpenOptions, File};
use std::io::{Read, Write};
use std::str::FromStr;

use chrono::prelude::*;

use rascam::SimpleCamera;
use pathdiff::diff_paths;

const DATETIME_FMT: &str = "%F_%H-%M-%S_%z";
const DATE_FMT: &str = "%F_%z";
const TIME_FMT: &str = "%H-%M-%S_%z";

fn main() {
    let path: PathBuf = "/home/pi/Dokumente/thermometer-config.yaml".into();
    let mut file = OpenOptions::new().read(true).open(path).expect("Could not open config file");
    let mut config_str = String::new();
    file.read_to_string(&mut config_str).expect("Could not read config file to string");

    let conf: config::Config = serde_yaml::from_str(config_str.as_str())
        .expect("Could not deserialize config");

    let output_path = PathBuf::from_str(conf.output_path()).unwrap();
    if !output_path.exists() {
        bunt::println!("{$blue}Creating output directory {[green]}{/$}", output_path.to_str().unwrap());
        std::fs::create_dir_all(&output_path);
    }

    let mut last_reading_time = Local::now();
    let mut data_output_path = output_path.clone();
    data_output_path.push("data");
    std::fs::create_dir_all(&data_output_path);
    data_output_path.push(format!("readings-{}.csv", last_reading_time.format(DATETIME_FMT)));
    let mut csv = csv::WriterBuilder::new()
        .delimiter(conf.delimiter())
        .from_path(&data_output_path).unwrap();

    let mut img_output_path = output_path.clone();
    img_output_path.push("img");
    let img_output_path = img_output_path;
    std::fs::create_dir_all(&img_output_path);

    let cam_start = conf.cam().cam_start();
    let cam_stop = conf.cam().cam_stop();
    let can_take_img = |time: NaiveTime| {
        if cam_start.is_none() || cam_stop.is_none() {
            true
        } else {
            time >= cam_start.unwrap() && time <= cam_stop.unwrap()
        }
    };

    let cams = cam::init().unwrap();

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
    headers.push("Bilder".to_owned());

    csv.write_record(&headers).unwrap();
    csv.flush().unwrap();

    loop {
        // TODO read all old images and scale them down
        // This way only the newest images are full size
        let now = Local::now();
        if last_reading_time.day() != now.day() {
            last_reading_time = now;
            data_output_path.set_file_name(format!("readings_{}.csv", last_reading_time.format(DATETIME_FMT)));
            csv = csv::WriterBuilder::new()
                .delimiter(conf.delimiter()) // TODO
                .from_path(&data_output_path).unwrap();
            csv.write_record(&headers).unwrap();
        } else {
            last_reading_time = now;
        }

        let mut curr_img_path = img_output_path.clone();
        curr_img_path.push(now.format(DATE_FMT).to_string());
        std::fs::create_dir_all(&curr_img_path);

        let mut readings = Vec::with_capacity(conf.sensors().len());

        println!("-------------------------");
        bunt::println!("Reading sensors for {} on {}", now.format("%T"), now.format("%d.%m.%Y"));

        for _ in 0..conf.sensors().len() {
            readings.push(String::new());
            readings.push(String::new());
        }

        let mut to_test: Vec<(usize, sensors::Sensor)> = conf.sensors().iter().map(|s| s.clone()).enumerate().collect();
        let mut tries: u64 = 0;

        'big_one: loop {
            if tries > 0 {
                bunt::println!("Retry #{[yellow]} of {[yellow]} max", tries, conf.max_sensor_retries());
            }
            let mut failed = Vec::new();
            let instant = std::time::Instant::now();

            for (idx, sensor) in to_test.iter() {
                match sensor.description() {
                    Some(s) => bunt::println!("Reading sensor {[yellow]}", s),
                    None => bunt::println!("Reading sensor on pin {[yellow]}", sensor.pin())
                }
                let reading = sensor.read();
                match reading {
                    Err(e) => {
                        bunt::eprintln!("{$red}Reading error{/$}: {:#?}", e);
                        if let Some(ele) = readings.get_mut(idx * 2) {
                            *ele = format!("Error: {:#?}", e);
                        }
                        if let Some(ele) = readings.get_mut(idx * 2 + 1) {
                            *ele = format!("Error: {:#?}", e);
                        }
                        failed.push((*idx, sensor.clone()));
                    },
                    Ok(o) => {
                        bunt::println!("Reading: t: {[green]} h: {[green]}", o.temperature, o.humidity);
                        if let Some(ele) = readings.get_mut(idx * 2) {
                            *ele = o.temperature.to_string();
                        }
                        if let Some(ele) = readings.get_mut(idx * 2 + 1) {
                            *ele = o.humidity.to_string();
                        }
                    },
                }
            }

            if !failed.is_empty() {
                bunt::eprintln!("{$red}Failed to read {[yellow]} sensors{/$}", failed.len());
                for (_, s) in failed.iter() {
                    if let Some(desc) = s.description() {
                        eprintln!("- {} -> {:?}", desc, s.sensor())
                    } else {
                        eprintln!("- Pin {} -> {:?}", s.pin(), s.sensor())
                    }
                }
            }

            to_test.clear();
            to_test = failed;

            tries += 1;
            if tries > conf.max_sensor_retries() || to_test.is_empty() {
                break 'big_one;
            }

            let to_add = Duration::from_secs(conf.min_read_time()).checked_sub(instant.elapsed());
            if let Some(time) = to_add {
                std::thread::sleep(time)
            }
        }

        let mut img_locs = vec![];
        if can_take_img(now.time()) {
            for cam in cams.iter() {
                bunt::println!("Camera {[blue]} is taking an image now", cam.name());
                match cam.take_and_save(
                    &curr_img_path,
                    format!("img_{}_{}", cam.name(), now.format(TIME_FMT)).as_str()
                ) {
                    Ok(path) => {
                        img_locs.push(data_output_path.parent()
                            .map(|par| pathdiff::diff_paths(&path, par))
                            .flatten()
                            .map(|rel| rel.to_string_lossy().into_owned())
                            .unwrap_or(path.to_string_lossy().into_owned())
                        );
                    },
                    Err(e) => {
                        bunt::println!("{$red}Error{/$}: couldn't take image ({[yellow]:?})", e);
                    }
                }
            }
        } else {
            bunt::println!("{$yellow}Info{/$}: Skipped taking an image due to the current time and config.");
            img_locs.push("Kamera aufgrund von Einstellungen übersprungen".to_owned());
        }
        // TODO somehow retry sensors that failed
        let mut record = vec![last_reading_time.to_string()];
        for r in readings {
            record.push(r);
        }

        let mut locs = String::new();
        for l in img_locs {
            locs += l.as_str();
            locs.push('\n');
        }
        record.push(locs);

        csv.write_record(&record).unwrap();
        csv.flush().unwrap();

        bunt::println!("Done reading sensors for {} on {}", now.format("%T"), now.format("%d.%m.%Y"));

        std::thread::sleep(Duration::from_secs(conf.read_interval()))
    }
}
