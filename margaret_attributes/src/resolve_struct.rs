use syn::Type;

use crate::canonical_path::CanonicalPath;
use crate::resolution::Resolution;
use crate::resolution_index::ResolutionIndex;

pub fn resolve_struct(declared: &Type, resolution: &ResolutionIndex) -> Option<CanonicalPath> {
    let Type::Path(type_path) = declared else {
        return None;
    };

    match resolution.resolve(&type_path.path) {
        Resolution::Resolved(model) => Some(model),
        _ => None,
    }
}
