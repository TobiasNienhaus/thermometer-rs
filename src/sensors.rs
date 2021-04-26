use serde::{Serialize, Deserialize};
pub use dht_lib::Sensor as DhtSensor;

#[derive(Debug, Serialize, Deserialize)]
pub struct SensorCollection {
    sensors: Vec<Sensor>
}

impl SensorCollection {
    pub fn sensors(&self) -> &[Sensor] {
        self.sensors.as_slice()
    }

    pub fn test() -> SensorCollection {
        SensorCollection {
            sensors: (0u8..12).map(|i| {
                Sensor {
                    sensor: DhtSensor::Dht11,
                    pin: i,
                    update_interval: Some(((i as u64) * 12) % 10),
                    description: Some("Hello".to_owned())
                }
            }).collect()
        }
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
        Sensor::Dht11 => 1,
        Sensor::Dht22 => 2
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Sensor {
    #[serde(with = "DhtSensorDef")]
    sensor: DhtSensor,
    pin: u8,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    update_interval: Option<u64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    description: Option<String>
}

impl Sensor {
    pub fn sensor(&self) -> &DhtSensor {
        &self.sensor
    }

    pub fn pin(&self) -> u8 {
        self.pin
    }

    pub fn update_interval(&self) -> u64 {
        self.update_interval.map(|v| {
            std::cmp::max(v, min_update_interval(&self.sensor))
        }).unwrap_or(min_update_interval(&self.sensor))
    }

    pub fn description(&self) -> Option<&str> {
        self.description.as_deref()
    }
}
