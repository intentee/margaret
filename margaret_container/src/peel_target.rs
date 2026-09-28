use syn::Path;
use syn::Type;
use syn::TypeParamBound;
use syn::TypeTraitObject;

use margaret_attributes::attribute_index::AttributeIndex;
use margaret_attributes::indexed_item::IndexedItem;
use margaret_attributes::standard_library_item::StandardLibraryItem;
use margaret_syn_type_peeling::peel_standard_wrapper::peel_standard_wrapper;

fn is_marker_trait(index: &AttributeIndex, item: &IndexedItem, path: &Path) -> bool {
    matches!(
        index
            .resolve_item_path(item, path)
            .as_ref()
            .and_then(StandardLibraryItem::from_canonical),
        Some(StandardLibraryItem::Send | StandardLibraryItem::Sync)
    )
}

fn peel_pointee(index: &AttributeIndex, item: &IndexedItem, pointee: &Type) -> Option<Path> {
    match pointee {
        Type::Path(type_path) => Some(type_path.path.clone()),
        Type::TraitObject(trait_object) => single_trait_bound(index, item, trait_object),
        _ => None,
    }
}

fn single_trait_bound(
    index: &AttributeIndex,
    item: &IndexedItem,
    trait_object: &TypeTraitObject,
) -> Option<Path> {
    let mut resolved: Option<Path> = None;

    for bound in &trait_object.bounds {
        if let TypeParamBound::Trait(trait_bound) = bound {
            if is_marker_trait(index, item, &trait_bound.path) {
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

pub(crate) fn peel_target(
    index: &AttributeIndex,
    item: &IndexedItem,
    parameter_type: &Type,
) -> Option<Path> {
    peel_standard_wrapper(
        index,
        item,
        parameter_type,
        &[StandardLibraryItem::Arc, StandardLibraryItem::Rc],
    )
    .and_then(|pointee| peel_pointee(index, item, pointee))
}

#[cfg(test)]
mod tests {
    use syn::Type;

    use margaret_attributes_tests::indexed_source::IndexedSource;

    use super::peel_target;

    fn label(parameter_type: &str) -> &'static str {
        let indexed =
            IndexedSource::new("use std::rc::Rc;\nuse std::sync::Arc;\n\nstruct Probe;\n");
        let parsed: Type = syn::parse_str(parameter_type).expect("a type fixture parses");

        match peel_target(&indexed.index, indexed.item("Probe"), &parsed) {
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
    fn rejects_a_same_named_pointer_of_another_crate() {
        assert_eq!(label("pointers::Arc<Config>"), "none");
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
