//! Checks for configured environment parameters to be
//! listed in NetBox. Things like the site, vm-cluster, device role, etc.

use thanix_client::util::ThanixClient;

use crate::{
    NazaraError,
    configuration::parser::{ConfigData, MachineConfig},
    error::NazaraResult,
};

/// Check if some NetBox objects exist.
///
/// # Parameters
///
/// * `client: &ThanixClient` - API client instance.
/// * `config: &ConfigData` - Configuration read from config file.
///
/// # Returns
///
/// `Ok(())` or `NazaraError` depending on operation outcome.
pub fn check_environment_objects(client: &ThanixClient, config: &ConfigData) -> NazaraResult<()> {
    status!("Checking configured NetBox entities exist...");

    match &config.machine {
        MachineConfig::Device(device_config) => {
            status!(
                "Checking device configuration: site={}, role={}, device_type={}",
                device_config.site,
                device_config.role,
                device_config.device_type
            );

            if device_config.site > 0 {
                check_site(client, device_config.site)?;
            }
            if device_config.role > 0 {
                check_device_role(client, device_config.role)?;
            }
            if device_config.device_type > 0 {
                check_device_type(client, device_config.device_type)?;
            }
        }
        MachineConfig::VM(vm_config) => {
            status!("Checking VM configuration: cluster={}", vm_config.cluster);

            if vm_config.cluster > 0 {
                check_vm_cluster(client, vm_config.cluster)?;
            }
        }
    }

    info!("All configured entities exist in NetBox");
    Ok(())
}

/// Check if given site, specified in the config file, is present.
///
/// # Parameters
/// * `client: &ThanixClient` - API client instance.
/// * `site_id: i64` - ID of the site to search for.
///
/// # Returns
///
/// `Ok(())` if the site has been found, returns `NazaraError::UnexpectedResponse` otherwise.
fn check_site(client: &ThanixClient, site_id: i64) -> NazaraResult<()> {
    status!("Checking site with ID {}...", site_id);
    match thanix_client::paths::dcim_sites_retrieve(client, site_id)? {
        thanix_client::paths::DcimSitesRetrieveResponse::Http200(site) => {
            let name = site.name.as_deref().unwrap_or("unknown");
            info!("Site '{}' (ID: {}) exists", name, site_id);
            Ok(())
        }
        thanix_client::paths::DcimSitesRetrieveResponse::Other(response) => {
            Err(NazaraError::UnexpectedResponse(response))
        }
    }
}

/// Check if given device_role, specified in the config file, is present.
///
/// # Parameters
/// * `client: &ThanixClient` - API client instance.
/// * `site_id: i64` - ID of the device_role to search for.
///
/// # Returns
///
/// `Ok(())` if the device_role has been found, returns `NazaraError::UnexpectedResponse` otherwise.
fn check_device_role(client: &ThanixClient, role_id: i64) -> NazaraResult<()> {
    status!("Checking device role with ID {}...", role_id);
    match thanix_client::paths::dcim_device_roles_retrieve(client, role_id)? {
        thanix_client::paths::DcimDeviceRolesRetrieveResponse::Http200(role) => {
            let name = role.name.as_deref().unwrap_or("unknown");
            info!("Device role '{}' (ID: {}) exists", name, role_id);
            Ok(())
        }
        thanix_client::paths::DcimDeviceRolesRetrieveResponse::Other(response) => {
            Err(NazaraError::UnexpectedResponse(response))
        }
    }
}

/// Check if given device_type, specified in the config file, is present.
///
/// # Parameters
/// * `client: &ThanixClient` - API client instance.
/// * `site_id: i64` - ID of the device_type to search for.
///
/// # Returns
///
/// `Ok(())` if the device_type has been found, returns `NazaraError::UnexpectedResponse` otherwise.
fn check_device_type(client: &ThanixClient, device_type_id: i64) -> NazaraResult<()> {
    status!("Checking device type with ID {}...", device_type_id);
    match thanix_client::paths::dcim_device_types_retrieve(client, device_type_id)? {
        thanix_client::paths::DcimDeviceTypesRetrieveResponse::Http200(device_type) => {
            let model = device_type.model.as_deref().unwrap_or("unknown");
            info!("Device type '{}' (ID: {}) exists", model, device_type_id);
            Ok(())
        }
        thanix_client::paths::DcimDeviceTypesRetrieveResponse::Other(response) => {
            Err(NazaraError::UnexpectedResponse(response))
        }
    }
}

/// Check if given vm_cluster, specified in the config file, is present.
///
/// # Parameters
/// * `client: &ThanixClient` - API client instance.
/// * `site_id: i64` - ID of the vm_cluster to search for.
///
/// # Returns
///
/// `Ok(())` if the vm_cluster has been found, returns `NazaraError::UnexpectedResponse` otherwise.
fn check_vm_cluster(client: &ThanixClient, cluster_id: i64) -> NazaraResult<()> {
    status!("Checking VM cluster with ID {}...", cluster_id);
    match thanix_client::paths::virtualization_clusters_retrieve(client, cluster_id)? {
        thanix_client::paths::VirtualizationClustersRetrieveResponse::Http200(cluster) => {
            let name = cluster.name.as_deref().unwrap_or("unknown");
            info!("VM cluster '{}' (ID: {}) exists", name, cluster_id);
            Ok(())
        }
        thanix_client::paths::VirtualizationClustersRetrieveResponse::Other(response) => {
            Err(NazaraError::UnexpectedResponse(response))
        }
    }
}
