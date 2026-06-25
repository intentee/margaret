use std::path::Path;

use crate::attribute_error::AttributeError;
use crate::attribute_selector::AttributeSelector;
use crate::canonical_path::CanonicalPath;
use crate::indexed_item::IndexedItem;
use crate::matched_attribute::MatchedAttribute;
use crate::module_walker::ModuleWalker;

pub struct AttributeIndex {
    items: Vec<IndexedItem>,
}

impl AttributeIndex {
    pub fn from_crate_root(
        crate_name: &str,
        source_directory: &Path,
    ) -> Result<Self, AttributeError> {
        let items = ModuleWalker::walk_crate(crate_name, source_directory)?;

        Ok(Self { items })
    }

    pub fn items(&self) -> &[IndexedItem] {
        &self.items
    }

    pub fn struct_paths(&self) -> Vec<CanonicalPath> {
        self.items
            .iter()
            .filter(|item| item.kind().is_struct())
            .map(|item| item.canonical_path().clone())
            .collect()
    }

    pub fn select(&self, selector: &AttributeSelector) -> Vec<MatchedAttribute<'_>> {
        let mut selected = Vec::new();

        for item in &self.items {
            for attribute in item.attributes() {
                if selector.matches(attribute.path()) {
                    selected.push(MatchedAttribute::new(item, attribute));
                }
            }
        }

        selected
    }
}
