use serde::{Serialize, Deserialize};
use super::sensors;

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    #[serde(default="default_decimal_separator", skip_serializing_if = "Option::is_none")]
    decimal_separator: Option<String>,
    decimal_places: usize,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    read_interval: Option<u64>,
    #[serde(default="default_delimiter")]
    delimiter: char,
    #[serde(default="default_sensor_retries")]
    max_sensor_retries: u64,
    sensors: Vec<sensors::Sensor>,
    output_path: String,
    cam: CamConfig
}

fn default_sensor_retries() -> u64 { 0 }

fn test_sensor_retries(val: u64) -> bool {
    val == 0
}

fn default_decimal_separator() -> Option<String> {
    Some(".".to_owned())
}

fn default_delimiter() -> char {
    return ',';
}

impl Config {
    pub fn num_formatter(&self) -> locale::Numeric {
        if let Some(sep) = &self.decimal_separator {
            locale::Numeric::new(sep.as_str(), "")
        } else {
            locale::Numeric::english()
        }
    }

    pub fn decimal_places(&self) -> usize {
        self.decimal_places
    }

    pub fn sensors(&self) -> &[sensors::Sensor] {
        self.sensors.as_slice()
    }

    pub fn read_interval(&self) -> u64 {
        self.read_interval.unwrap_or(self.min_read_time())
    }

    pub fn min_read_time(&self) -> u64 {
        self.sensors.iter().map(|s| super::sensors::min_update_interval(&s.sensor())).sum()
    }

    pub fn delimiter(&self) -> u8 {
        self.delimiter as u8
    }

    pub fn output_path(&self) -> &str {
        self.output_path.as_str()
    }

    pub fn cam(&self) -> &CamConfig {
        &self.cam
    }

    pub fn max_sensor_retries(&self) -> u64 {
        self.max_sensor_retries
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CamConfig {
    // %H:%M:%S
    time_format: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    start: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    stop: Option<String>
}

impl CamConfig {
    pub fn cam_start(&self) -> Option<chrono::NaiveTime> {
        if let Some(start) = &self.start {
            let format =
                chrono::format::strftime::StrftimeItems::new(self.time_format.as_str());
            let mut parsed = chrono::format::Parsed::new();
            let res = chrono::format::parse(&mut parsed, start.as_str(), format).ok();
            if res.is_some() {
                parsed.to_naive_time().ok()
            } else {
                None
            }
        } else {
            None
        }
    }

    pub fn cam_stop(&self) -> Option<chrono::NaiveTime> {
        if let Some(stop) = &self.stop {
            let format =
                chrono::format::strftime::StrftimeItems::new(self.time_format.as_str());
            let mut parsed = chrono::format::Parsed::new();
            let res = chrono::format::parse(&mut parsed, stop.as_str(), format).ok();
            if res.is_some() {
                parsed.to_naive_time().ok()
            } else {
                None
            }
        } else {
            None
        }
    }
}