use syn::Type;

use margaret_attributes::canonical_path::CanonicalPath;
use margaret_attributes::resolution::Resolution;
use margaret_attributes::resolve_unique::resolve_unique;

pub(crate) fn resolve_struct(
    declared: &Type,
    candidates: &[CanonicalPath],
) -> Option<CanonicalPath> {
    let Type::Path(type_path) = declared else {
        return None;
    };

    match resolve_unique(&type_path.path, candidates) {
        Resolution::Resolved(model) => Some(model),
        _ => None,
    }
}
