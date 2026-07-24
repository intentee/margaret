use crate::attribute_error::AttributeError;
use crate::attribute_selector::AttributeSelector;
use crate::indexed_item::IndexedItem;
use crate::matched_attribute::MatchedAttribute;
use crate::select_unique_attribute::select_unique_attribute;

pub struct AttributeQuery<'index> {
    item: &'index IndexedItem,
}

impl<'index> AttributeQuery<'index> {
    #[must_use]
    pub fn new(item: &'index IndexedItem) -> Self {
        Self { item }
    }

    pub fn find(
        &self,
        selector: &AttributeSelector,
    ) -> Result<Option<MatchedAttribute<'index>>, AttributeError> {
        let unique = select_unique_attribute(self.item.attributes(), selector, || {
            self.item.canonical_path().to_string()
        })?;

        Ok(unique.map(|attribute| MatchedAttribute::new(self.item, attribute)))
    }

    #[must_use]
    pub fn find_all(&self, selector: &AttributeSelector) -> Vec<MatchedAttribute<'index>> {
        self.item
            .attributes()
            .iter()
            .filter(|attribute| selector.matches(attribute.path()))
            .map(|attribute| MatchedAttribute::new(self.item, attribute))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use syn::Attribute;
    use syn::parse_quote;

    use crate::attribute_query::AttributeQuery;
    use crate::attribute_selector::AttributeSelector;
    use crate::canonical_path::CanonicalPath;
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
        )
    }

    fn selector(input: &str) -> AttributeSelector {
        AttributeSelector::parse(input).expect("the selector parses")
    }

    #[test]
    fn find_returns_the_single_matching_sibling() {
        let item = item(vec![parse_quote!(#[singleton])]);

        assert!(
            AttributeQuery::new(&item)
                .find(&selector("singleton"))
                .expect("the lookup succeeds")
                .is_some()
        );
    }

    #[test]
    fn find_returns_none_when_no_sibling_matches() {
        let item = item(vec![parse_quote!(#[singleton])]);

        assert!(
            AttributeQuery::new(&item)
                .find(&selector("does_not_exist"))
                .expect("the lookup succeeds")
                .is_none()
        );
    }

    #[test]
    fn find_rejects_a_repeated_sibling() {
        let item = item(vec![parse_quote!(#[singleton]), parse_quote!(#[singleton])]);
        let message = AttributeQuery::new(&item)
            .find(&selector("singleton"))
            .err()
            .expect("a repeated sibling is rejected")
            .to_string();

        assert!(message.contains("is repeated"));
    }

    #[test]
    fn find_all_returns_every_matching_sibling() {
        let item = item(vec![parse_quote!(#[singleton]), parse_quote!(#[singleton])]);

        assert_eq!(
            AttributeQuery::new(&item)
                .find_all(&selector("singleton"))
                .len(),
            2
        );
    }
}
