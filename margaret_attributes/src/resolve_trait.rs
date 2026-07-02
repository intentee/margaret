use syn::GenericArgument;
use syn::PathArguments;
use syn::PathSegment;
use syn::Type;
use syn::TypeParamBound;
use syn::TypeTraitObject;

use crate::canonical_path::CanonicalPath;
use crate::resolution::Resolution;
use crate::resolution_index::ResolutionIndex;

fn boxed_type_argument(segment: &PathSegment) -> Option<&Type> {
    let PathArguments::AngleBracketed(arguments) = &segment.arguments else {
        return None;
    };

    match arguments.args.first() {
        Some(GenericArgument::Type(inner)) => Some(inner),
        _ => None,
    }
}

fn peel_trait_object(declared: &Type) -> Option<&TypeTraitObject> {
    match declared {
        Type::TraitObject(trait_object) => Some(trait_object),
        Type::Path(type_path) => {
            let segment = type_path
                .path
                .segments
                .last()
                .expect("a type path has at least one segment");

            if segment.ident != "Box" {
                return None;
            }

            peel_trait_object(boxed_type_argument(segment)?)
        }
        _ => None,
    }
}

pub fn resolve_trait(declared: &Type, resolution: &ResolutionIndex) -> Option<CanonicalPath> {
    let trait_object = peel_trait_object(declared)?;
    let written = trait_object
        .bounds
        .iter()
        .find_map(|bound| match bound {
            TypeParamBound::Trait(trait_bound) => Some(&trait_bound.path),
            _ => None,
        })
        .expect("a trait object names at least one trait");

    match resolution.resolve(written) {
        Resolution::Resolved(resolved) => Some(resolved),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use syn::Type;
    use syn::parse_quote;

    use super::resolve_trait;
    use crate::canonical_path::CanonicalPath;
    use crate::resolution_index::ResolutionIndex;

    fn marker_traits() -> ResolutionIndex {
        ResolutionIndex::new([CanonicalPath::new(vec![
            "crate".to_string(),
            "View".to_string(),
        ])])
    }

    fn resolved(declared: Type) -> Option<String> {
        resolve_trait(&declared, &marker_traits()).map(|path| path.to_string())
    }

    #[test]
    fn resolves_a_boxed_marker_trait_object() {
        assert_eq!(
            resolved(parse_quote!(Box<dyn View>)),
            Some("crate::View".to_string())
        );
    }

    #[test]
    fn resolves_a_bare_marker_trait_object() {
        assert_eq!(
            resolved(parse_quote!(dyn View)),
            Some("crate::View".to_string())
        );
    }

    #[test]
    fn resolves_a_marker_trait_listed_after_a_lifetime() {
        assert_eq!(
            resolved(parse_quote!(Box<dyn 'static + View>)),
            Some("crate::View".to_string())
        );
    }

    #[test]
    fn ignores_a_plain_struct_return() {
        assert_eq!(resolved(parse_quote!(Response)), None);
    }

    #[test]
    fn ignores_a_tuple_return() {
        assert_eq!(resolved(parse_quote!((String, String))), None);
    }

    #[test]
    fn ignores_a_box_without_arguments() {
        assert_eq!(resolved(parse_quote!(Box)), None);
    }

    #[test]
    fn ignores_a_box_without_a_type_argument() {
        assert_eq!(resolved(parse_quote!(Box<'static>)), None);
    }

    #[test]
    fn ignores_an_unknown_marker_trait() {
        assert_eq!(resolved(parse_quote!(Box<dyn Unknown>)), None);
    }
}
