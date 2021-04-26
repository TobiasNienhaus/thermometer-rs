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
                    description: None
                }
            }).collect()
        }
    }
}

#[derive(Serialize, Deserialize)]
#[serde(remote = "DhtSensor")]
enum DhtSensorDef {
    Dht11,
    Dht22
}

#[derive(Serialize, Deserialize)]
pub struct Sensor {
    #[serde(with = "DhtSensorDef")]
    sensor: DhtSensor,
    pin: u8,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    description: Option<String>
}
