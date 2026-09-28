use std::fs;

use tempfile::TempDir;
use tempfile::tempdir;

use margaret_attributes::attribute_index::AttributeIndex;
use margaret_attributes::attribute_index_builder::AttributeIndexBuilder;
use margaret_attributes::crate_root::CrateRoot;
use margaret_attributes::indexed_item::IndexedItem;

pub struct IndexedSource {
    pub index: AttributeIndex,
    _directory: TempDir,
}

impl IndexedSource {
    /// # Panics
    ///
    /// Panics when the crate it writes cannot be created or indexed.
    #[must_use]
    pub fn new(lib_source: &str) -> Self {
        let directory = tempdir().expect("a temporary crate directory is created");
        let source = directory.path().join("src");

        fs::create_dir_all(&source).expect("the src directory exists");
        fs::write(source.join("lib.rs"), lib_source).expect("lib.rs is written");

        let index = AttributeIndexBuilder::new()
            .index_crate(&CrateRoot::new("crate", source))
            .expect("the crate is indexed")
            .build();

        Self {
            index,
            _directory: directory,
        }
    }

    /// # Panics
    ///
    /// Panics when no indexed item carries the identifier.
    #[must_use]
    pub fn item(&self, identifier: &str) -> &IndexedItem {
        self.index
            .items()
            .iter()
            .find(|item| item.identifier() == identifier)
            .expect("the item is indexed")
    }
}
