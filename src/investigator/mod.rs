//! This module checks and prepares relevant aspects on the NetBox side.
mod environment;
mod tags;

use thanix_client::util::ThanixClient;

use crate::{NazaraResult, configuration::parser::ConfigData};

/// Run all checks of the NetBox environment.
///
/// # Parameters
///
/// * `client: &ThanixClient` - API client instance.
/// * `config: &ConfigData` - Configuration data.
/// * `prepare_environment: bool` - Whether to create missing entities.
///
/// # Returns
///
/// `Ok(())` or `NazaraError` depending on operation outcome.
pub fn check_environment(
    client: &ThanixClient,
    config: &ConfigData,
    prepare_environment: bool,
) -> NazaraResult<()> {
    println!("Running NetBox environment checks before registration...");

    environment::check_environment_objects(client, config)?;

    tags::ensure_required_tags(client, prepare_environment)?;

    Ok(())
}
