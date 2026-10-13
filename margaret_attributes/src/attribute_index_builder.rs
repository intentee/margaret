use std::collections::BTreeSet;

use crate::attribute_error::AttributeError;
use crate::attribute_index::AttributeIndex;
use crate::crate_root::CrateRoot;
use crate::indexed_item::IndexedItem;
use crate::module_walker::ModuleWalker;
use crate::path_resolver::PathResolver;
use crate::walk_output::WalkOutput;

#[derive(Default)]
pub struct AttributeIndexBuilder {
    excluded_root_modules: BTreeSet<String>,
    items: Vec<IndexedItem>,
    resolver: PathResolver,
}

impl AttributeIndexBuilder {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    #[must_use]
    pub fn build(self) -> AttributeIndex {
        AttributeIndex::new(self.items, self.resolver)
    }

    #[must_use]
    pub fn exclude_root_module(mut self, name: impl Into<String>) -> Self {
        self.excluded_root_modules.insert(name.into());

        self
    }

    /// # Errors
    ///
    /// Returns `AttributeError` propagated from the work it performs.
    pub fn index_crate(mut self, crate_root: &CrateRoot) -> Result<Self, AttributeError> {
        let WalkOutput { items, resolver } = ModuleWalker::walk_crate(
            &crate_root.name,
            &crate_root.source_directory,
            &self.excluded_root_modules,
        )?;

        self.items.extend(items);
        self.items
            .sort_by(|left, right| left.canonical_path().cmp(right.canonical_path()));
        self.resolver.extend(resolver);

        match self.items.windows(2).find_map(|pair| match pair {
            [left, right] if left.canonical_path() == right.canonical_path() => Some(left),
            _ => None,
        }) {
            Some(duplicate) => Err(AttributeError::DuplicateCanonicalPath {
                path: duplicate.canonical_path().to_string(),
            }),
            None => Ok(self),
        }
    }
}
