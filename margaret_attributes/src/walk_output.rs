use crate::indexed_item::IndexedItem;
use crate::path_resolver::PathResolver;

pub(crate) struct WalkOutput {
    pub(crate) items: Vec<IndexedItem>,
    pub(crate) resolver: PathResolver,
}
