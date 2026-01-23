//! This module checks and prepares relevant aspects on the NetBox side.
pub mod environment;
pub mod tags;

use thanix_client::util::ThanixClient;

use crate::{NazaraResult, configuration::parser::ConfigData, info};

/// Run all checks of the NetBox environment.
pub fn check_environment(
    client: &ThanixClient,
    config: &ConfigData,
    prepare_environment: bool,
) -> NazaraResult<()> {
    info!("Running NetBox environment checks before registration...");

    environment::check_environment_objects(client, config)?;
    tags::ensure_required_tags(client, prepare_environment)?;

    Ok(())
}
