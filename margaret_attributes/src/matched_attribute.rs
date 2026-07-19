use crate::attribute_args::AttributeArgs;
use crate::attribute_error::AttributeError;
use crate::attribute_selector::AttributeSelector;
use crate::format_path::format_path;
use crate::indexed_item::IndexedItem;

pub struct MatchedAttribute<'index> {
    attribute_index: usize,
    item: &'index IndexedItem,
}

impl<'index> MatchedAttribute<'index> {
    pub(crate) fn new(item: &'index IndexedItem, attribute_index: usize) -> Self {
        Self {
            attribute_index,
            item,
        }
    }

    pub fn args(&self) -> Result<&'index AttributeArgs, AttributeError> {
        self.item.attribute_args(self.attribute_index)
    }

    #[must_use]
    pub fn item(&self) -> &'index IndexedItem {
        self.item
    }

    #[must_use]
    pub fn matches(&self, selector: &AttributeSelector) -> bool {
        selector.matches(self.item.attributes()[self.attribute_index].path())
    }

    #[must_use]
    pub fn path(&self) -> String {
        format_path(self.item.attributes()[self.attribute_index].path())
    }
}

#[cfg(test)]
mod tests {
    use syn::Attribute;
    use syn::parse_quote;

    use crate::attribute_selector::AttributeSelector;
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
        )
    }

    #[test]
    fn path_is_the_attributes_written_path() {
        let item = item_bearing(parse_quote!(#[ns::tagged]));
        let matched = MatchedAttribute::new(&item, 0);

        assert_eq!(matched.path(), "ns::tagged");
    }

    #[test]
    fn args_parse_the_matched_attribute() {
        let item = item_bearing(parse_quote!(#[singleton]));
        let matched = MatchedAttribute::new(&item, 0);

        assert!(matched.args().expect("the arguments parse").is_empty());
    }

    #[test]
    fn args_are_parsed_once_and_reused() {
        let item = item_bearing(parse_quote!(#[responds_to_http(method = "get")]));
        let matched = MatchedAttribute::new(&item, 0);

        let first = matched.args().expect("the arguments parse");
        let second = matched.args().expect("the arguments parse");

        assert!(std::ptr::eq(first, second));
    }

    #[test]
    fn matches_tests_the_attribute_against_a_selector() {
        let item = item_bearing(parse_quote!(#[tagged(crate::markers::Tag)]));
        let matched = MatchedAttribute::new(&item, 0);

        assert!(matched.matches(&AttributeSelector::parse("tagged").expect("a valid selector")));
        assert!(!matched.matches(&AttributeSelector::parse("traced").expect("a valid selector")));
    }
}
