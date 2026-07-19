use std::collections::HashMap;

use crate::attribute_error::AttributeError;
use crate::attribute_index::AttributeIndex;
use crate::canonical_path::CanonicalPath;
use crate::crate_root::CrateRoot;
use crate::indexed_item::IndexedItem;
use crate::module_imports::ModuleImports;
use crate::module_walker::ModuleWalker;
use crate::walk_output::WalkOutput;

#[derive(Default)]
pub struct AttributeIndexBuilder {
    imports: HashMap<CanonicalPath, ModuleImports>,
    items: Vec<IndexedItem>,
}

impl AttributeIndexBuilder {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    #[must_use]
    pub fn build(self) -> AttributeIndex {
        AttributeIndex::new(self.items, self.imports)
    }

    pub fn index_crate(mut self, crate_root: &CrateRoot) -> Result<Self, AttributeError> {
        let WalkOutput { imports, items } =
            ModuleWalker::walk_crate(&crate_root.name, &crate_root.source_directory)?;

        self.imports.extend(imports);
        self.items.extend(items);

        Ok(self)
    }
}
