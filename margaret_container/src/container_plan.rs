use crate::collection_table::CollectionTable;
use crate::provider::Provider;

pub(crate) struct ContainerPlan {
    pub(crate) collections: CollectionTable,
    pub(crate) providers: Vec<Provider>,
}
