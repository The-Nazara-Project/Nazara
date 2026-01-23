use crate::{NazaraError, NazaraResult};
use crate::{failure, info, success, warn};
use thanix_client::util::ThanixClient;

const REQUIRED_TAGS: &[(&str, &str)] = &[("nazara", "blue"), ("dhcp", "orange")];

/// Ensure required NetBox tags exist.
///
/// If `prepare_environment` is true, missing tags will be created.
pub fn ensure_required_tags(client: &ThanixClient, prepare_environment: bool) -> NazaraResult<()> {
    todo!("Implement tag search/creation");
}
