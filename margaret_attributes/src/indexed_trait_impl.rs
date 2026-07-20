use syn::Path;

use crate::indexed_associated_type::IndexedAssociatedType;

pub struct IndexedTraitImpl {
    associated_types: Vec<IndexedAssociatedType>,
    module_path: Vec<String>,
    trait_path: Path,
}

impl IndexedTraitImpl {
    pub(crate) fn new(
        trait_path: Path,
        module_path: Vec<String>,
        mut associated_types: Vec<IndexedAssociatedType>,
    ) -> Self {
        associated_types.sort_by(|left, right| left.name().cmp(right.name()));

        Self {
            associated_types,
            module_path,
            trait_path,
        }
    }

    #[must_use]
    pub fn associated_type(&self, name: &str) -> Option<&IndexedAssociatedType> {
        self.associated_types
            .iter()
            .find(|associated_type| associated_type.name() == name)
    }

    #[must_use]
    pub fn associated_types(&self) -> &[IndexedAssociatedType] {
        &self.associated_types
    }

    #[must_use]
    pub fn module_path(&self) -> &[String] {
        &self.module_path
    }

    #[must_use]
    pub fn trait_path(&self) -> &Path {
        &self.trait_path
    }
}
