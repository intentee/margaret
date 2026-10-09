use margaret_attribute_arguments::attribute_args::AttributeArgs;
use margaret_attribute_arguments::format_path::format_path;

use crate::attribute_error::AttributeError;
use crate::indexed_attribute::IndexedAttribute;
use crate::indexed_item::IndexedItem;

pub struct MatchedAttribute<'index> {
    attribute: &'index IndexedAttribute,
    item: &'index IndexedItem,
}

impl<'index> MatchedAttribute<'index> {
    pub(crate) fn new(item: &'index IndexedItem, attribute: &'index IndexedAttribute) -> Self {
        Self { attribute, item }
    }

    /// # Errors
    ///
    /// Returns `AttributeError` propagated from the work it performs.
    pub fn args(&self) -> Result<&'index AttributeArgs, AttributeError> {
        self.attribute.args()
    }

    #[must_use]
    pub fn item(&self) -> &'index IndexedItem {
        self.item
    }

    #[must_use]
    pub fn path(&self) -> String {
        format_path(self.attribute.path())
    }
}

#[cfg(test)]
mod tests {
    use std::ptr;

    use syn::Attribute;
    use syn::parse_quote;

    use crate::canonical_path::CanonicalPath;
    use crate::indexed_item::IndexedItem;
    use crate::item_kind::ItemKind;
    use crate::matched_attribute::MatchedAttribute;
    use crate::struct_shape::StructShape;

    fn item_bearing(attribute: Attribute) -> IndexedItem {
        IndexedItem::new(
            ItemKind::Struct(StructShape::Unit),
            "Service".to_string(),
            CanonicalPath::new(vec!["crate".to_string(), "Service".to_string()]),
            vec![attribute],
            Vec::new(),
            Vec::new(),
            false,
        )
    }

    #[test]
    fn path_is_the_attributes_written_path() {
        let item = item_bearing(parse_quote!(#[ns::tagged]));
        let matched = MatchedAttribute::new(&item, &item.attributes()[0]);

        assert_eq!(matched.path(), "ns::tagged");
    }

    #[test]
    fn args_parse_the_matched_attribute() {
        let item = item_bearing(parse_quote!(#[singleton]));
        let matched = MatchedAttribute::new(&item, &item.attributes()[0]);

        assert!(matched.args().expect("the arguments parse").is_empty());
    }

    #[test]
    fn args_are_parsed_once_and_reused() {
        let item = item_bearing(parse_quote!(#[responds_to_http(method = RouteMethod::Get)]));
        let matched = MatchedAttribute::new(&item, &item.attributes()[0]);

        let first = matched.args().expect("the arguments parse");
        let second = matched.args().expect("the arguments parse");

        assert!(ptr::eq(first, second));
    }
}
