use std::path::Path;

use crate::attribute_error::AttributeError;
use crate::attribute_holder::AttributeHolder;
use crate::attribute_selector::AttributeSelector;
use crate::matched_attribute::MatchedAttribute;
use crate::module_walker::ModuleWalker;

pub struct AttributeIndex {
    holders: Vec<AttributeHolder>,
}

impl AttributeIndex {
    pub fn from_crate_root(
        crate_name: &str,
        source_directory: &Path,
    ) -> Result<Self, AttributeError> {
        let holders = ModuleWalker::walk_crate(crate_name, source_directory)?;

        Ok(Self { holders })
    }

    pub fn holders(&self) -> &[AttributeHolder] {
        &self.holders
    }

    pub fn select(&self, selector: &AttributeSelector) -> Vec<MatchedAttribute<'_>> {
        let mut selected = Vec::new();

        for holder in &self.holders {
            for attribute in holder.attributes() {
                if selector.matches(attribute.path()) {
                    selected.push(MatchedAttribute::new(holder, attribute));
                }
            }
        }

        selected
    }
}
