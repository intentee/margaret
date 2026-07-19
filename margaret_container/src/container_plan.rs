use std::collections::BTreeMap;

use margaret_attributes::canonical_path::CanonicalPath;

use crate::collection_table::CollectionTable;
use crate::provider::Provider;

pub(crate) struct ContainerPlan {
    pub(crate) collections: CollectionTable,
    pub(crate) providers: BTreeMap<CanonicalPath, Provider>,
}
