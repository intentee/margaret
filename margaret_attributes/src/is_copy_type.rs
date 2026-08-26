use crate::attribute_index::AttributeIndex;
use crate::canonical_path::CanonicalPath;
use crate::indexed_item::IndexedItem;
use crate::is_copy_primitive::is_copy_primitive;
use crate::is_std_copy_type::is_std_copy_type;

#[must_use]
pub fn is_copy_type(index: &AttributeIndex, canonical: &CanonicalPath) -> bool {
    if matches!(canonical.segments(), [name] if is_copy_primitive(name)) {
        return true;
    }

    if is_std_copy_type(canonical) {
        return true;
    }

    index.item(canonical).is_some_and(IndexedItem::is_copy)
}
