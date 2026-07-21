use syn::Type;

use crate::single_generic_argument::single_generic_argument;

pub(crate) fn option_inner(ty: &Type) -> Option<&Type> {
    let Type::Path(type_path) = ty else {
        return None;
    };

    match type_path.path.segments.last() {
        Some(segment) if segment.ident == "Option" => single_generic_argument(segment),
        _ => None,
    }
}
