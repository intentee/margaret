use syn::Type;

use margaret_attributes::attribute_index::AttributeIndex;
use margaret_attributes::canonical_path::CanonicalPath;
use margaret_attributes::indexed_item::IndexedItem;

fn cancellation_token_path() -> CanonicalPath {
    CanonicalPath::new(vec![
        "tokio_util".to_string(),
        "sync".to_string(),
        "CancellationToken".to_string(),
    ])
}

#[must_use]
pub fn is_cancellation_token(index: &AttributeIndex, item: &IndexedItem, declared: &Type) -> bool {
    !matches!(declared, Type::Reference(_))
        && index.resolve_item_type(item, declared) == Some(cancellation_token_path())
}
