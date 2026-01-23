//! Checks for configured environment parameters to be
//! listed in NetBox. Things like the site, vm-cluster, device role, etc.

use thanix_client::util::ThanixClient;

use crate::{
    NazaraError,
    configuration::parser::{ConfigData, MachineConfig},
    error::NazaraResult,
    info,
};

pub fn check_environment_objects(client: &ThanixClient, config: &ConfigData) -> NazaraResult<()> {
    todo!("Implement check for site, device_role, device_type and vm cluster");
}
