use syn::Type;

use margaret_attributes::attribute_index::AttributeIndex;
use margaret_attributes::indexed_item::IndexedItem;
use margaret_attributes::standard_library_item::StandardLibraryItem;

use crate::single_generic_argument::single_generic_argument;

#[must_use]
pub fn peel_standard_wrapper<'declared>(
    index: &AttributeIndex,
    item: &IndexedItem,
    declared: &'declared Type,
    wrappers: &[StandardLibraryItem],
) -> Option<&'declared Type> {
    let Type::Path(type_path) = declared else {
        return None;
    };

    index
        .resolve_item_type(item, declared)
        .as_ref()
        .and_then(StandardLibraryItem::from_canonical)
        .filter(|wrapper| wrappers.contains(wrapper))
        .and_then(|_| type_path.path.segments.last())
        .and_then(single_generic_argument)
}

#[cfg(test)]
mod tests {
    use quote::quote;
    use syn::Type;

    use margaret_attributes::standard_library_item::StandardLibraryItem;
    use margaret_attributes_tests::indexed_source::IndexedSource;

    use super::peel_standard_wrapper;

    fn peeled(
        lib_source: &str,
        type_source: &str,
        wrappers: &[StandardLibraryItem],
    ) -> Option<String> {
        let indexed = IndexedSource::new(&format!("{lib_source}\nstruct Probe;\n"));
        let declared: Type = syn::parse_str(type_source).expect("the type fixture parses");

        peel_standard_wrapper(&indexed.index, indexed.item("Probe"), &declared, wrappers)
            .map(|inner| quote!(#inner).to_string())
    }

    #[test]
    fn peels_the_prelude_option() {
        assert_eq!(
            peeled("", "Option<Node>", &[StandardLibraryItem::Option]).as_deref(),
            Some("Node")
        );
    }

    #[test]
    fn peels_an_imported_pointer_that_is_requested() {
        assert_eq!(
            peeled(
                "use std::sync::Arc;",
                "Arc<Node>",
                &[StandardLibraryItem::Arc, StandardLibraryItem::Rc]
            )
            .as_deref(),
            Some("Node")
        );
    }

    #[test]
    fn ignores_a_user_defined_type_named_after_a_wrapper() {
        assert!(
            peeled(
                "struct Option<Inner>(Inner);",
                "Option<Node>",
                &[StandardLibraryItem::Option]
            )
            .is_none()
        );
    }

    #[test]
    fn ignores_a_wrapper_that_is_not_requested() {
        assert!(peeled("", "Box<Node>", &[StandardLibraryItem::Option]).is_none());
    }

    #[test]
    fn ignores_a_non_path_type() {
        assert!(peeled("", "[u8; 4]", &[StandardLibraryItem::Option]).is_none());
    }

    #[test]
    fn ignores_a_wrapper_with_more_than_one_argument() {
        assert!(peeled("", "Option<Node, Edge>", &[StandardLibraryItem::Option]).is_none());
    }
}
