use crate::attribute_error::AttributeError;
use crate::attribute_holder::AttributeHolder;
use crate::attribute_selector::AttributeSelector;
use crate::matched_attribute::MatchedAttribute;

pub struct AttributeQuery<'index> {
    holder: &'index AttributeHolder,
}

impl<'index> AttributeQuery<'index> {
    pub fn new(holder: &'index AttributeHolder) -> Self {
        Self { holder }
    }

    pub fn find_all(&self, selector: &AttributeSelector) -> Vec<MatchedAttribute<'index>> {
        let mut matches = Vec::new();

        for attribute in self.holder.attributes() {
            if selector.matches(attribute.path()) {
                matches.push(MatchedAttribute::new(self.holder, attribute));
            }
        }

        matches
    }

    pub fn find(
        &self,
        selector: &AttributeSelector,
    ) -> Result<Option<MatchedAttribute<'index>>, AttributeError> {
        let mut matches = self.find_all(selector);

        if matches.len() > 1 {
            return Err(AttributeError::RepeatedAttribute {
                attribute_path: selector.display_path(),
                target: self.holder.target_path(),
            });
        }

        Ok(matches.pop())
    }
}

#[cfg(test)]
mod tests {
    use syn::Attribute;
    use syn::parse_quote;

    use crate::attribute_holder::AttributeHolder;
    use crate::attribute_query::AttributeQuery;
    use crate::attribute_selector::AttributeSelector;
    use crate::canonical_path::CanonicalPath;
    use crate::indexed_item::IndexedItem;
    use crate::item_kind::ItemKind;
    use crate::struct_shape::StructShape;

    fn holder(attributes: Vec<Attribute>) -> AttributeHolder {
        AttributeHolder::Item(IndexedItem::new(
            ItemKind::Struct(StructShape::Unit),
            "Service".to_string(),
            CanonicalPath::new(vec!["crate".to_string(), "Service".to_string()]),
            attributes,
        ))
    }

    fn selector(input: &str) -> AttributeSelector {
        AttributeSelector::parse(input).expect("the selector parses")
    }

    #[test]
    fn find_returns_the_single_matching_sibling() {
        let holder = holder(vec![parse_quote!(#[singleton])]);

        assert!(
            AttributeQuery::new(&holder)
                .find(&selector("singleton"))
                .expect("the lookup succeeds")
                .is_some()
        );
    }

    #[test]
    fn find_returns_none_when_no_sibling_matches() {
        let holder = holder(vec![parse_quote!(#[singleton])]);

        assert!(
            AttributeQuery::new(&holder)
                .find(&selector("does_not_exist"))
                .expect("the lookup succeeds")
                .is_none()
        );
    }

    #[test]
    fn find_rejects_a_repeated_sibling() {
        let holder = holder(vec![parse_quote!(#[singleton]), parse_quote!(#[singleton])]);
        let message = AttributeQuery::new(&holder)
            .find(&selector("singleton"))
            .err()
            .expect("a repeated sibling is rejected")
            .to_string();

        assert!(message.contains("is repeated"));
    }

    #[test]
    fn find_all_returns_every_matching_sibling() {
        let holder = holder(vec![parse_quote!(#[singleton]), parse_quote!(#[singleton])]);

        assert_eq!(
            AttributeQuery::new(&holder)
                .find_all(&selector("singleton"))
                .len(),
            2
        );
    }
}
