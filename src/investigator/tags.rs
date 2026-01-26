use crate::{Machine, NazaraError, NazaraResult};
use crate::{failure, info, success, warn};
use thanix_client::paths::{
    DcimDeviceRolesRetrieveResponse, DcimDeviceTypesRetrieveResponse, DcimRacksRetrieveResponse,
    DcimSitesRetrieveResponse, ExtrasTagsCreateResponse, ExtrasTagsListQuery,
    ExtrasTagsListResponse, VirtualizationClustersRetrieveResponse, extras_tags_create,
    extras_tags_list,
};
use thanix_client::types::{Tag, TagRequest};
use thanix_client::util::ThanixClient;

const REQUIRED_TAGS: &[&str] = &["nazara", "dhcp"];

/// Ensure required NetBox tags exist.
///
/// If `prepare_environment` is true, missing tags will be created.
pub fn ensure_required_tags(client: &ThanixClient, prepare_environment: bool) -> NazaraResult<()> {
    todo!("Implement tag search/creation");
}

/// Check whether the entities configured in the devices config file exist.
///
/// E.g. racks, site, vm_cluster, ...
///
/// # Parameters
///
/// * `client: &ThanixClient` - The client instance to use.
/// * `machine: &Machine` - The representation of machine information
/// * `prepare_environment: bool` - Whether or not to create the given enities under a generic name.
///
/// # Returns
/// `Ok(())` or `Err` depending on operation outcome.
pub fn check_entities_exist(
    client: &ThanixClient,
    machine: &Machine,
    prepare_environment: bool,
) -> NazaraResult<()> {
    todo!("Implement entity checks.")
}

/// Ensure that the specified tags exist.
///
/// # Parameters
/// * `client: &ThanixClient` - API client instance.
/// * `names: Vec<String>` - Names of the tags we expect.
///
/// # Returns
/// Ok(()) when the tags are actually present. Returns an `Err` if the tags
/// are either not present or something goes wrong during communication with NetBox.
fn tags_exist(client: &ThanixClient, names: Vec<String>) -> NazaraResult<()> {
    let payload: ExtrasTagsListQuery = ExtrasTagsListQuery {
        name: Some(names),
        ..Default::default()
    };

    match extras_tags_list(client, payload)? {
        ExtrasTagsListResponse::Http200(tag_list) => {
            tag_list
                .results
                .ok_or(NazaraError::NetBoxMissingField(
                    "PaginatedTagList".into(),
                    "results".into(),
                ))?
                .first()
                .map(|x: &Tag| x.name.clone());

            return Ok(());
        }
        ExtrasTagsListResponse::Other(response) => Err(NazaraError::UnexpectedResponse(response)),
    }
}

/// Create the expected tag.
///
/// # Parameters
/// * `client: &ThanixClient` - API client instance to use.
/// * `name: String` - The name of the tag to create.
///
/// # Returns
/// The ID of the newly created tag.
fn create_tag(client: &ThanixClient, name: &str) -> NazaraResult<i64> {
    let payload: TagRequest = TagRequest {
        name: name.to_owned(),
        ..Default::default()
    };

    match extras_tags_create(client, payload)? {
        ExtrasTagsCreateResponse::Http201(result) => {
            success!(
                "Tag '{}' created successfully. New tag ID is: '{}'!",
                name,
                result.id,
            );
            Ok(result.id)
        }
        ExtrasTagsCreateResponse::Other(response) => Err(NazaraError::UnexpectedResponse(response)),
    }
}

fn rack_exists(state: &ThanixClient, id: i64) -> NazaraResult<()> {
    match thanix_client::paths::dcim_racks_retrieve(state, id)? {
        DcimRacksRetrieveResponse::Http200(_) => Ok(()),
        DcimRacksRetrieveResponse::Other(response) => {
            Err(NazaraError::UnexpectedResponse(response))
        }
    }
}

fn device_type_exists(state: &ThanixClient, id: i64) -> NazaraResult<()> {
    match thanix_client::paths::dcim_device_types_retrieve(state, id)? {
        DcimDeviceTypesRetrieveResponse::Http200(_) => Ok(()),
        DcimDeviceTypesRetrieveResponse::Other(response) => {
            Err(NazaraError::UnexpectedResponse(response))
        }
    }
}

fn device_role_exists(state: &ThanixClient, id: i64) -> NazaraResult<()> {
    match thanix_client::paths::dcim_device_roles_retrieve(state, id)? {
        DcimDeviceRolesRetrieveResponse::Http200(_) => Ok(()),
        DcimDeviceRolesRetrieveResponse::Other(response) => {
            Err(NazaraError::UnexpectedResponse(response))
        }
    }
}

fn device_site_exists(state: &ThanixClient, id: i64) -> NazaraResult<()> {
    match thanix_client::paths::dcim_sites_retrieve(state, id)? {
        DcimSitesRetrieveResponse::Http200(_) => Ok(()),
        DcimSitesRetrieveResponse::Other(response) => {
            Err(NazaraError::UnexpectedResponse(response))
        }
    }
}

fn vm_cluster_exists(state: &ThanixClient, id: i64) -> NazaraResult<()> {
    match thanix_client::paths::virtualization_clusters_retrieve(state, id)? {
        VirtualizationClustersRetrieveResponse::Http200(_) => Ok(()),
        VirtualizationClustersRetrieveResponse::Other(response) => {
            Err(NazaraError::UnexpectedResponse(response))
        }
    }
}
