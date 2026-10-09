use margaret_attributes::canonical_path::CanonicalPath;
use margaret_attributes::identifier::Identifier;
use margaret_umbrella_path::umbrella_item_path::umbrella_item_path;

use crate::accepted_client_item::AcceptedClientItem;
use crate::accepted_clients_module_name::ACCEPTED_CLIENTS_MODULE_NAME;
use crate::clients_module_name::CLIENTS_MODULE_NAME;

#[must_use]
pub fn accepted_client_item_path(anchor: &Identifier, item: AcceptedClientItem) -> CanonicalPath {
    umbrella_item_path(&[
        ACCEPTED_CLIENTS_MODULE_NAME,
        CLIENTS_MODULE_NAME,
        anchor.field(),
        item.type_name(),
    ])
}
