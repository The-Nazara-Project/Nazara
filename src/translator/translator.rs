//! # The Translator module.
//!
//! This module will translate the information from the `collector` module into the `JSON`-Format the Netbox API
//! understands.

use collectors::{dmi_collector, network_collector};
use serde::{Deserialize, Serialize};

use crate::collectors;

pub fn serialize_to_json(
    dmi_info: dmi_collector::DmiInformation,
    network_info: Vec<network_collector::NetworkInformation>,
) -> String {
    let system = match serde_json::to_string(&dmi_info) {
        Ok(json) => json,
        Err(err) => panic!("{}", err),
    };
    let network = match serde_json::to_string(&network_info) {
        Ok(json) => json,
        Err(err) => panic!("{}", err),
    };
    let result: String = system + &network;

    result
}
