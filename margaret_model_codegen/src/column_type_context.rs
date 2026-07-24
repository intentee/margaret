use std::collections::HashSet;

use margaret_attributes::attribute_index::AttributeIndex;
use margaret_attributes::canonical_path::CanonicalPath;
use margaret_attributes::indexed_item::IndexedItem;

pub(crate) struct ColumnTypeContext<'context> {
    pub(crate) attribute_index: &'context AttributeIndex,
    pub(crate) item: &'context IndexedItem,
    pub(crate) validated_enums: &'context mut HashSet<CanonicalPath>,
}
