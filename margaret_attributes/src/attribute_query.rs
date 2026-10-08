use crate::framework_attribute::FrameworkAttribute;
use crate::indexed_item::IndexedItem;
use crate::matched_attribute::MatchedAttribute;
use crate::select_framework_attributes::select_framework_attributes;

pub struct AttributeQuery<'index> {
    item: &'index IndexedItem,
}

impl<'index> AttributeQuery<'index> {
    #[must_use]
    pub fn new(item: &'index IndexedItem) -> Self {
        Self { item }
    }

    #[must_use]
    pub fn find_all_framework(
        &self,
        framework_attribute: FrameworkAttribute,
    ) -> Vec<MatchedAttribute<'index>> {
        select_framework_attributes(self.item.attributes(), framework_attribute)
            .into_iter()
            .map(|attribute| MatchedAttribute::new(self.item, attribute))
            .collect()
    }

    #[must_use]
    pub fn find_framework(
        &self,
        framework_attribute: FrameworkAttribute,
    ) -> Option<MatchedAttribute<'index>> {
        self.item
            .attributes()
            .iter()
            .find(|attribute| attribute.framework_attribute() == Some(framework_attribute))
            .map(|attribute| MatchedAttribute::new(self.item, attribute))
    }
}

#[cfg(test)]
mod tests {
    use syn::Attribute;
    use syn::parse_quote;

    use crate::attribute_query::AttributeQuery;
    use crate::canonical_path::CanonicalPath;
    use crate::framework_attribute::FrameworkAttribute;
    use crate::indexed_item::IndexedItem;
    use crate::item_kind::ItemKind;
    use crate::struct_shape::StructShape;

    fn item(attributes: Vec<Attribute>) -> IndexedItem {
        IndexedItem::new(
            ItemKind::Struct(StructShape::Unit),
            "Service".to_string(),
            CanonicalPath::new(vec!["crate".to_string(), "Service".to_string()]),
            attributes,
            Vec::new(),
            Vec::new(),
            false,
        )
    }

    #[test]
    fn find_returns_the_single_matching_sibling() {
        let item = item(vec![parse_quote!(#[singleton])]);

        assert!(
            AttributeQuery::new(&item)
                .find_framework(FrameworkAttribute::Singleton)
                .is_some()
        );
    }

    #[test]
    fn find_returns_none_when_no_sibling_matches() {
        let item = item(vec![parse_quote!(#[singleton])]);

        assert!(
            AttributeQuery::new(&item)
                .find_framework(FrameworkAttribute::Model)
                .is_none()
        );
    }

    #[test]
    fn find_all_returns_every_matching_sibling() {
        let item = item(vec![
            parse_quote!(#[middleware(logged)]),
            parse_quote!(#[middleware(traced)]),
        ]);

        assert_eq!(
            AttributeQuery::new(&item)
                .find_all_framework(FrameworkAttribute::Middleware)
                .len(),
            2
        );
    }
}
