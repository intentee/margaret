use margaret_attributes::canonical_path::CanonicalPath;
use margaret_umbrella_path::umbrella_item_path::umbrella_item_path;

use crate::constants::OIDC_PROVIDER_MODULE_NAME;
use crate::oidc_provider_item::OidcProviderItem;

#[must_use]
pub fn oidc_provider_item_path(item: OidcProviderItem) -> CanonicalPath {
    umbrella_item_path(&[OIDC_PROVIDER_MODULE_NAME, item.type_name()])
}
