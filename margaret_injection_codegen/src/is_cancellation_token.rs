use syn::Type;

use margaret_attributes::attribute_index::AttributeIndex;
use margaret_attributes::indexed_item::IndexedItem;

use crate::cancellation_token_path::cancellation_token_path;

#[must_use]
pub fn is_cancellation_token(index: &AttributeIndex, item: &IndexedItem, declared: &Type) -> bool {
    !matches!(declared, Type::Reference(_))
        && index.resolve_item_type(item, declared) == Some(cancellation_token_path())
}
