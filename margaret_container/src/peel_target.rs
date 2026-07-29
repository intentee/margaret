use syn::Path;
use syn::Type;
use syn::TypeParamBound;
use syn::TypeTraitObject;

use margaret_syn_type_peeling::single_generic_argument::single_generic_argument;

fn peel_arc_inner(inner: &Type) -> Option<Path> {
    match inner {
        Type::Path(type_path) => Some(type_path.path.clone()),
        Type::TraitObject(trait_object) => single_trait_bound(trait_object),
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

pub(crate) fn peel_target(parameter_type: &Type) -> Option<Path> {
    let Type::Path(type_path) = parameter_type else {
        return None;
    };

    match type_path.path.segments.last() {
        Some(segment) if segment.ident == "Arc" || segment.ident == "Rc" => {
            peel_arc_inner(single_generic_argument(segment)?)
        }
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use syn::Type;

    use super::peel_target;

    fn label(parameter_type: &str) -> &'static str {
        let parsed: Type = syn::parse_str(parameter_type).expect("a type fixture parses");

        match peel_target(&parsed) {
            Some(_) => "single",
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
}
