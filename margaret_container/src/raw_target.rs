use syn::GenericArgument;
use syn::Path;
use syn::PathArguments;
use syn::PathSegment;
use syn::Type;
use syn::TypeParamBound;
use syn::TypeTraitObject;

pub(crate) enum RawTarget {
    Collection(Path),
    Single(Path),
}

pub(crate) fn peel_target(parameter_type: &Type) -> Option<RawTarget> {
    let Type::Path(type_path) = parameter_type else {
        return None;
    };
    let segment = type_path
        .path
        .segments
        .last()
        .expect("a type path has at least one segment");

    if segment.ident == "Arc" || segment.ident == "Rc" {
        peel_arc_inner(single_generic_argument(segment)?)
    } else if segment.ident == "Vec" {
        Some(RawTarget::Collection(peel_collection_inner(
            single_generic_argument(segment)?,
        )?))
    } else {
        None
    }
}

fn peel_arc_inner(inner: &Type) -> Option<RawTarget> {
    match inner {
        Type::Path(type_path) => Some(RawTarget::Single(type_path.path.clone())),
        Type::TraitObject(trait_object) => {
            Some(RawTarget::Single(single_trait_bound(trait_object)?))
        }
        _ => None,
    }
}

fn peel_collection_inner(inner: &Type) -> Option<Path> {
    let Type::Path(type_path) = inner else {
        return None;
    };
    let segment = type_path
        .path
        .segments
        .last()
        .expect("a type path has at least one segment");

    if segment.ident != "Arc" && segment.ident != "Rc" {
        return None;
    }

    let Type::TraitObject(trait_object) = single_generic_argument(segment)? else {
        return None;
    };

    single_trait_bound(trait_object)
}

fn single_generic_argument(segment: &PathSegment) -> Option<&Type> {
    let PathArguments::AngleBracketed(arguments) = &segment.arguments else {
        return None;
    };

    if arguments.args.len() != 1 {
        return None;
    }

    match arguments
        .args
        .first()
        .expect("a single-argument list has a first argument")
    {
        GenericArgument::Type(generic_type) => Some(generic_type),
        _ => None,
    }
}

fn single_trait_bound(trait_object: &TypeTraitObject) -> Option<Path> {
    let mut resolved: Option<Path> = None;

    for bound in &trait_object.bounds {
        if let TypeParamBound::Trait(trait_bound) = bound {
            if is_marker_trait(&trait_bound.path) {
                continue;
            }
            if resolved.is_some() {
                return None;
            }
            resolved = Some(trait_bound.path.clone());
        }
    }

    resolved
}

fn is_marker_trait(path: &Path) -> bool {
    path.is_ident("Send") || path.is_ident("Sync")
}

#[cfg(test)]
mod tests {
    use syn::Type;

    use super::RawTarget;
    use super::peel_target;

    fn label(parameter_type: &str) -> &'static str {
        let parsed: Type = syn::parse_str(parameter_type).expect("a type fixture parses");

        match peel_target(&parsed) {
            Some(RawTarget::Single(_)) => "single",
            Some(RawTarget::Collection(_)) => "collection",
            None => "none",
        }
    }

    #[test]
    fn peels_arc_of_concrete() {
        assert_eq!(label("Arc<Config>"), "single");
    }

    #[test]
    fn peels_rc_of_concrete() {
        assert_eq!(label("Rc<Config>"), "single");
    }

    #[test]
    fn peels_arc_of_trait_object() {
        assert_eq!(label("Arc<dyn Greeter>"), "single");
    }

    #[test]
    fn peels_vec_of_arc_trait_object() {
        assert_eq!(label("Vec<Arc<dyn Plugin>>"), "collection");
    }

    #[test]
    fn peels_vec_of_rc_trait_object() {
        assert_eq!(label("Vec<Rc<dyn Plugin>>"), "collection");
    }

    #[test]
    fn rejects_bare_type() {
        assert_eq!(label("Config"), "none");
    }

    #[test]
    fn rejects_unknown_wrapper() {
        assert_eq!(label("Box<Config>"), "none");
    }

    #[test]
    fn rejects_reference() {
        assert_eq!(label("&Config"), "none");
    }

    #[test]
    fn rejects_arc_of_tuple() {
        assert_eq!(label("Arc<(Config, Logger)>"), "none");
    }

    #[test]
    fn peels_arc_of_send_sync_trait_object() {
        assert_eq!(label("Arc<dyn Greeter + Send + Sync>"), "single");
    }

    #[test]
    fn peels_arc_of_trait_object_with_lifetime() {
        assert_eq!(label("Arc<dyn Greeter + 'static>"), "single");
    }

    #[test]
    fn rejects_arc_of_multi_trait_object() {
        assert_eq!(label("Arc<dyn Greeter + Plugin>"), "none");
    }

    #[test]
    fn rejects_arc_of_marker_only_trait_object() {
        assert_eq!(label("Arc<dyn Send + Sync>"), "none");
    }

    #[test]
    fn rejects_arc_without_arguments() {
        assert_eq!(label("Arc"), "none");
    }

    #[test]
    fn rejects_arc_with_multiple_arguments() {
        assert_eq!(label("Arc<Config, Logger>"), "none");
    }

    #[test]
    fn rejects_arc_with_lifetime_argument() {
        assert_eq!(label("Arc<'static>"), "none");
    }

    #[test]
    fn rejects_vec_without_arguments() {
        assert_eq!(label("Vec"), "none");
    }

    #[test]
    fn rejects_vec_of_bare_type() {
        assert_eq!(label("Vec<Config>"), "none");
    }

    #[test]
    fn rejects_vec_of_reference() {
        assert_eq!(label("Vec<&Plugin>"), "none");
    }

    #[test]
    fn rejects_vec_of_arc_concrete() {
        assert_eq!(label("Vec<Arc<Config>>"), "none");
    }

    #[test]
    fn rejects_vec_of_arc_without_arguments() {
        assert_eq!(label("Vec<Arc>"), "none");
    }
}
