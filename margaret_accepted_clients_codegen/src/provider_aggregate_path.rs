use margaret_attributes::canonical_path::CanonicalPath;
use margaret_umbrella_path::umbrella_item_path::umbrella_item_path;

use crate::accepted_clients_module_name::ACCEPTED_CLIENTS_MODULE_NAME;
use crate::provider_aggregate::ProviderAggregate;

#[must_use]
pub fn provider_aggregate_path(aggregate: ProviderAggregate) -> CanonicalPath {
    umbrella_item_path(&[
        ACCEPTED_CLIENTS_MODULE_NAME,
        aggregate.module_name(),
        aggregate.constant_name(),
    ])
}
