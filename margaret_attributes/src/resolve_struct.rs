use syn::Type;

use crate::canonical_path::CanonicalPath;
use crate::resolution::Resolution;
use crate::resolve_unique::resolve_unique;

pub fn resolve_struct(
    declared: &Type,
    candidates: &[CanonicalPath],
    referencing_root: &str,
) -> Option<CanonicalPath> {
    let Type::Path(type_path) = declared else {
        return None;
    };

    match resolve_unique(&type_path.path, candidates, referencing_root) {
        Resolution::Resolved(model) => Some(model),
        _ => None,
    }
}
