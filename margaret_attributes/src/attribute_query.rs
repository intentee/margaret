use crate::attribute_selector::AttributeSelector;
use crate::error::AttributeError;
use crate::indexed_item::IndexedItem;
use crate::matched_attribute::MatchedAttribute;

pub struct AttributeQuery<'index> {
    item: &'index IndexedItem,
}

impl<'index> AttributeQuery<'index> {
    pub(crate) fn new(item: &'index IndexedItem) -> Self {
        Self { item }
    }

    pub fn find_all(&self, selector: &AttributeSelector) -> Vec<MatchedAttribute<'index>> {
        let mut matches = Vec::new();

        for attribute in self.item.attributes() {
            if selector.matches(attribute.path()) {
                matches.push(MatchedAttribute::new(self.item, attribute));
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
                item_path: self.item.canonical_path().to_string(),
            });
        }

        Ok(matches.pop())
    }
}
