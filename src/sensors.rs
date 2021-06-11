use serde::{Serialize, Deserialize};
pub use dht_lib::Sensor as DhtSensor;
use dht_lib::{Reading, ReadingError};

#[derive(Debug, Serialize, Deserialize)]
#[serde(remote = "DhtSensor")]
enum DhtSensorDef {
    Dht11,
    Dht22
}

pub const fn min_update_interval(sensor: &DhtSensor) -> u64 {
    match sensor {
        DhtSensor::Dht11 => 1,
        DhtSensor::Dht22 => 2
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Sensor {
    // TODO maybe store last read point and then
    //  add another function to wait until the sensor is free to be read again
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
