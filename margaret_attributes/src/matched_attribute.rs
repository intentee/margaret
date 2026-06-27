use syn::Attribute;

use crate::attribute_args::AttributeArgs;
use crate::attribute_error::AttributeError;
use crate::attribute_selector::AttributeSelector;
use crate::format_path::format_path;
use crate::indexed_item::IndexedItem;

pub struct MatchedAttribute<'index> {
    attribute: &'index Attribute,
    item: &'index IndexedItem,
}

impl<'index> MatchedAttribute<'index> {
    pub(crate) fn new(item: &'index IndexedItem, attribute: &'index Attribute) -> Self {
        Self { attribute, item }
    }

    pub fn item(&self) -> &'index IndexedItem {
        self.item
    }

    pub fn path(&self) -> String {
        format_path(self.attribute.path())
    }

    pub fn matches(&self, selector: &AttributeSelector) -> bool {
        selector.matches(self.attribute.path())
    }

    pub fn args(&self) -> Result<AttributeArgs, AttributeError> {
        AttributeArgs::from_attribute(self.attribute)
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
    fn matches_tests_the_attribute_against_a_selector() {
        let item = item_bearing(parse_quote!(#[can(crate::action::Action::Read)]));
        let matched = MatchedAttribute::new(&item, &item.attributes()[0]);

        assert!(matched.matches(&AttributeSelector::parse("can").expect("a valid selector")));
        assert!(!matched.matches(&AttributeSelector::parse("traced").expect("a valid selector")));
    }
}
