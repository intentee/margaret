use margaret_attributes::attribute_error::AttributeError;
use margaret_attributes::attribute_index::AttributeIndex;
use margaret_attributes::attribute_index_builder::AttributeIndexBuilder;
use margaret_attributes::crate_root::CrateRoot;
use margaret_attributes::indexed_item::IndexedItem;

use crate::source_crate::SourceCrate;

pub struct IndexedSource {
    pub index: AttributeIndex,
}

impl IndexedSource {
    /// # Panics
    ///
    /// Panics when the crate it writes cannot be created or indexed.
    #[must_use]
    pub fn new(lib_source: &str) -> Self {
        Self::try_new(lib_source).expect("the crate is indexed")
    }

    /// # Errors
    ///
    /// Returns the `AttributeError` raised while indexing the crate.
    ///
    /// # Panics
    ///
    /// Panics when the crate it writes cannot be created.
    pub fn try_new(lib_source: &str) -> Result<Self, AttributeError> {
        let source_crate = SourceCrate::new(lib_source);

        Ok(Self {
            index: AttributeIndexBuilder::new()
                .index_crate(&CrateRoot::new("crate", source_crate.source_directory()))?
                .build(),
        })
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
