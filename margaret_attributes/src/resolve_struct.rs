use syn::Type;

use crate::canonical_path::CanonicalPath;
use crate::resolution::Resolution;
use crate::resolution_index::ResolutionIndex;

pub fn resolve_struct(
    declared: &Type,
    resolution: &ResolutionIndex,
    referencing_root: &str,
) -> Option<CanonicalPath> {
    let Type::Path(type_path) = declared else {
        return None;
    };

    match resolution.resolve(&type_path.path, referencing_root) {
        Resolution::Resolved(model) => Some(model),
        _ => None,
    }
}
