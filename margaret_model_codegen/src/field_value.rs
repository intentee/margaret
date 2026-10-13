use margaret_attributes::canonical_path::CanonicalPath;
use margaret_model::on_delete::OnDelete;

use crate::resolved_rust_type::ResolvedRustType;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum FieldValue {
    Enum {
        path: CanonicalPath,
        variants: Vec<String>,
    },
    Json {
        payload: ResolvedRustType,
    },
    Key {
        on_delete: OnDelete,
        target: CanonicalPath,
    },
    Scalar {
        rust_type: ResolvedRustType,
    },
}
