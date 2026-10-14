use syn::Type;

use margaret_attributes::attribute_index::AttributeIndex;
use margaret_attributes::canonical_path::CanonicalPath;

fn cancellation_token_path() -> CanonicalPath {
    CanonicalPath::new(vec![
        "tokio_util".to_string(),
        "sync".to_string(),
        "CancellationToken".to_string(),
    ])
}

#[must_use]
pub fn is_cancellation_token(
    index: &AttributeIndex,
    module_path: &[String],
    declared: &Type,
) -> bool {
    !matches!(declared, Type::Reference(_))
        && index.resolve_module_type(module_path, declared) == Some(cancellation_token_path())
}
