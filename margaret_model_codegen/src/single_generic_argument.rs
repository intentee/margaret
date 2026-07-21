use syn::GenericArgument;
use syn::PathArguments;
use syn::PathSegment;
use syn::Type;

pub(crate) fn single_generic_argument(segment: &PathSegment) -> Option<&Type> {
    let PathArguments::AngleBracketed(arguments) = &segment.arguments else {
        return None;
    };

    let mut arguments = arguments.args.iter();

    match (arguments.next(), arguments.next()) {
        (Some(GenericArgument::Type(generic_type)), None) => Some(generic_type),
        _ => None,
    }
}
