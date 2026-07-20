use syn::GenericArgument;
use syn::PathArguments;
use syn::PathSegment;
use syn::Type;

fn single_generic_argument(segment: &PathSegment) -> Option<&Type> {
    let PathArguments::AngleBracketed(arguments) = &segment.arguments else {
        return None;
    };

    let mut arguments = arguments.args.iter();

    match (arguments.next(), arguments.next()) {
        (Some(GenericArgument::Type(generic_type)), None) => Some(generic_type),
        _ => None,
    }
}

pub(crate) fn option_inner(ty: &Type) -> Option<&Type> {
    let Type::Path(type_path) = ty else {
        return None;
    };

    match type_path.path.segments.last() {
        Some(segment) if segment.ident == "Option" => single_generic_argument(segment),
        _ => None,
    }
}
