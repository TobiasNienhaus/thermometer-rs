use serde::{Serialize, Deserialize};
pub use dht_lib::Sensor as DhtSensor;
use dht_lib::{Reading, ReadingError};

#[derive(Debug, Serialize, Deserialize)]
pub struct SensorConfig {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    read_interval: Option<u64>,
    #[serde(default=",")]
    delimiter: char,
    sensors: Vec<Sensor>,
    output_path: String
}

impl SensorConfig {
    pub fn sensors(&self) -> &[Sensor] {
        self.sensors.as_slice()
    }

    pub fn read_interval(&self) -> u64 {
        self.read_interval.unwrap_or(self.min_read_time())
    }

    pub fn min_read_time(&self) -> u64 {
        self.sensors.iter().map(|s| min_update_interval(&s.sensor)).sum()
    }

    pub fn delimiter(&self) -> u8 {
        self.delimiter as u8
    }

    pub fn output_path(&self) -> &str {
        self.output_path.as_str()
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(remote = "DhtSensor")]
enum DhtSensorDef {
    Dht11,
    Dht22
}

const fn min_update_interval(sensor: &DhtSensor) -> u64 {
    match sensor {
        DhtSensor::Dht11 => 1,
        DhtSensor::Dht22 => 2
    }
}

// No idea what I wanted to use that for :D
// const MIN_READ_TIME_MS: u64 = 0;
// const MIN_READ_TIME_SEC: f64 = 1f64 / MIN_READ_TIME_MS;

#[derive(Debug, Serialize, Deserialize)]
pub struct Sensor {
    #[serde(with = "DhtSensorDef")]
    sensor: DhtSensor,
    pin: u8,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    description: Option<String>
}

impl Sensor {
    pub fn dht_11(pin: u8) -> Sensor {
        Sensor {
            sensor: DhtSensor::Dht11,
            pin,
            description: None
        }
    }

    pub fn dht_22(pin: u8) -> Sensor {
        Sensor {
            sensor: DhtSensor::Dht22,
            pin,
            description: None
        }
    }

    pub fn named_dht_11(pin: u8, name: &str) -> Sensor {
        Sensor {
            sensor: DhtSensor::Dht11,
            pin,
            description: Some(name.to_owned())
        }
    }

    pub fn named_dht_22(pin: u8, name: &str) -> Sensor {
        Sensor {
            sensor: DhtSensor::Dht22,
            pin,
            description: Some(name.to_owned())
        }
    }

    pub fn sensor(&self) -> &DhtSensor {
        &self.sensor
    }

    pub fn pin(&self) -> u8 {
        self.pin
    }

    pub fn description(&self) -> Option<&str> {
        self.description.as_deref()
    }

    pub fn read(&self) -> Result<Reading, ReadingError> {
        dht_lib::read(self.sensor, self.pin)
    }
}
