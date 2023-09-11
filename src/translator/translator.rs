//! # The Translator module.
//!
//! This module will translate the information from the `collector` module into the `JSON`-Format the Netbox API
//! understands.

use collectors::{dmi_collector, network_collector};
use serde::{Deserialize, Serialize};
use serde_json::{Result, Value};

use crate::collectors;
