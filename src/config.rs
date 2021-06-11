use serde::{Serialize, Deserialize};
use super::sensors;

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    read_interval: Option<u64>,
    #[serde(default="default_path")]
    delimiter: char,
    sensors: Vec<sensors::Sensor>,
    output_path: String,
    cam: CamConfig
}

fn default_path() -> char {
    return ',';
}

impl Config {
    pub fn sensors(&self) -> &[sensors::Sensor] {
        self.sensors.as_slice()
    }

    pub fn read_interval(&self) -> u64 {
        self.read_interval.unwrap_or(self.min_read_time())
    }

    pub fn min_read_time(&self) -> u64 {
        self.sensors.iter().map(|s| min_update_interval(&s.sensor())).sum()
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
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CamConfig {
    // %H:%M:%S
    date_format: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    start: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    stop: Option<String>
}

impl CamConfig {
    pub fn cam_start(&self) -> Option<chrono::NaiveTime> {
        let start = if self.start.is_some() {
            self.start.unwrap().as_str()
        } else {
            return None
        };
        let format =
            chrono::format::strftime::StrftimeItems::new(self.date_format.as_str());
        let mut parsed = chrono::format::Parsed::new();
        let res = chrono::format::parse(&mut parsed, start, format).ok();
        if res.is_some() {
            parsed.to_naive_time().ok()
        } else {
            None
        }
    }

    pub fn cam_stop(&self) -> Option<chrono::NaiveTime> {
        let start = if self.stop.is_some() {
            self.stop.unwrap().as_str()
        } else {
            return None
        };
        let format =
            chrono::format::strftime::StrftimeItems::new(self.date_format.as_str());
        let mut parsed = chrono::format::Parsed::new();
        let res = chrono::format::parse(&mut parsed, start, format).ok();
        if res.is_some() {
            parsed.to_naive_time().ok()
        } else {
            None
        }
    }
}