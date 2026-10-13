use margaret_attributes::canonical_path::CanonicalPath;
use margaret_attributes::tag::Tag;
use margaret_umbrella_path::umbrella_item_path::umbrella_item_path;

use crate::constants::OAUTH_CLIENTS_MODULE_NAME;
use crate::oauth_client_item::OAuthClientItem;

#[must_use]
pub fn oauth_client_item_path(tag: &Tag, item: OAuthClientItem) -> CanonicalPath {
    umbrella_item_path(&[
        OAUTH_CLIENTS_MODULE_NAME,
        &tag.to_string(),
        item.type_name(),
    ])
}
