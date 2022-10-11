use core::fmt;

use prost::{EncodeError, Message};

pub mod messages {
    // The name "messages" corresponds with the `package` name in the `.proto`
    include!(concat!(env!("OUT_DIR"), "/messages.rs"));
}

#[allow(dead_code)]
pub fn create_thermostat_state() -> messages::ThermostatState {
    messages::ThermostatState::default()
}

#[allow(dead_code)]
pub fn serialize_message(msg: &messages::ThermostatState) -> Result<Vec<u8>, EncodeError> {
    let mut buf = Vec::new();
    msg.encode(&mut buf)?;
    Ok(buf)
}

#[allow(dead_code)]
pub fn deserialize_message(buf: &[u8]) -> Result<messages::ThermostatState, prost::DecodeError> {
    messages::ThermostatState::decode(buf)
}

impl fmt::Display for messages::ThermostatState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}: air temp {}, rad temp {}",
            self.name, self.air_temp, self.rad_temp
        )
    }
}
