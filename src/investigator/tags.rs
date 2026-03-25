use crate::{constants::REQUIRED_TAGS, error::*};
use thanix_client::paths::{
    ExtrasTagsCreateResponse, ExtrasTagsListQuery, ExtrasTagsListResponse, extras_tags_create,
    extras_tags_list,
};
use thanix_client::types::TagRequest;
use thanix_client::util::ThanixClient;

/// Ensure that the required tags are present.
/// If they are not present, and `prepare_environment` is set, they are created.
///
/// # Parameters
/// * `client: &ThanixClient` - API client instance to use.
/// * `prepare_environment: bool` - Whether or not to create the expected tags.
///
/// # Returns
///
/// `Ok(())` if the tags are found, `NazaraError` otherwise. Except when `prepare_environment`
/// is passed. Then it attempts to create the tags in NetBox and escalates the operation result.
pub fn ensure_required_tags(client: &ThanixClient, prepare_environment: bool) -> NazaraResult<()> {
    status!("Checking for required NetBox tags...");

    for tag_name in REQUIRED_TAGS {
        match tag_exists(client, tag_name) {
            Ok(true) => {
                info!("Tag '{}' already exists", tag_name);
            }
            Ok(false) => {
                if prepare_environment {
                    match create_tag(client, tag_name) {
                        Ok(_) => {}
                        Err(e) => {
                            warn!("Failed to create tag '{}': {}", tag_name, e);
                        }
                    }
                } else {
                    warn!(
                        "Tag '{}' does not exist. Use --prepare-environment to create it.",
                        tag_name
                    );
                }
            }
            Err(e) => {
                warn!("Could not check if tag '{}' exists: {}", tag_name, e);
            }
        }
    }

    success!("Tag check complete");
    Ok(())
}

/// Check if given tag exists.
///
/// # Params
/// * `client: &ThanixClient` - API client instance to use.
/// * `name: &str` - The name of the tag to search for.
///
/// # Returns
///
/// `True` if the tag exists, `false` if it doesn't, or `NazaraError` on API errors.
fn tag_exists(client: &ThanixClient, name: &str) -> NazaraResult<bool> {
    let payload: ExtrasTagsListQuery = ExtrasTagsListQuery {
        name: Some(vec![name.to_string()]),
        ..Default::default()
    };

    match extras_tags_list(client, payload)? {
        ExtrasTagsListResponse::Http200(tag_list) => {
            let results = tag_list.results.ok_or(NazaraError::NetBoxMissingField(
                "PaginatedTagList".into(),
                "results".into(),
            ))?;
            Ok(!results.is_empty())
        }
        ExtrasTagsListResponse::Other(response) => {
            let status = response.status();
            if status.as_u16() == 400 {
                Ok(false)
            } else {
                Err(NazaraError::UnexpectedResponse(response))
            }
        }
    }
}

/// Create a tag in NetBox.
///
/// # Params
/// * `client: &ThanixClient` - API client instance.
/// * `name: &str` - Name of the tag to create.
///
/// # Returns
///
/// `Ok(i64)` if the creation is successful, otherwise `NazaraError`.
fn create_tag(client: &ThanixClient, name: &str) -> NazaraResult<i64> {
    let slug = name.to_lowercase();
    let color = match name {
        "nazara" => "03a9f4",
        "dhcp" => "9e9e9e",
        _ => "ffffff",
    };

    let payload: TagRequest = TagRequest {
        name: name.to_owned(),
        slug: slug,
        color: color.to_string(),
        description: String::new(),
        object_types: Vec::new(),
        weight: 0,
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
