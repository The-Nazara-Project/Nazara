use std::sync::LazyLock;
use thanix_client::types::NestedTagRequest;

pub const REQUIRED_TAGS: &[&str] = &["nazara", "dhcp"];

pub static NAZARA_TAG_REQUEST: LazyLock<NestedTagRequest> = LazyLock::new(|| NestedTagRequest {
    name: "nazara".to_string(),
    slug: "nazara".to_string(),
    color: "03a9f4".to_string(),
});

pub static DHCP_TAG_REQUEST: LazyLock<NestedTagRequest> = LazyLock::new(|| NestedTagRequest {
    name: "dhcp".to_string(),
    slug: "dhcp".to_string(),
    color: "9e9e9e".to_string(),
});
