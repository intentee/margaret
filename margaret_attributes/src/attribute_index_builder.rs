use crate::attribute_error::AttributeError;
use crate::attribute_index::AttributeIndex;
use crate::crate_root::CrateRoot;
use crate::indexed_item::IndexedItem;
use crate::module_walker::ModuleWalker;

#[derive(Default)]
pub struct AttributeIndexBuilder {
    items: Vec<IndexedItem>,
}

impl AttributeIndexBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn index_crate(mut self, crate_root: &CrateRoot) -> Result<Self, AttributeError> {
        self.items.extend(ModuleWalker::walk_crate(
            &crate_root.name,
            &crate_root.source_directory,
        )?);

        Ok(self)
    }

    pub fn build(self) -> AttributeIndex {
        AttributeIndex::new(self.items)
    }
}
