use syn::Attribute;

use crate::canonical_path::CanonicalPath;
use crate::item_kind::ItemKind;

pub struct IndexedItem {
    attributes: Vec<Attribute>,
    canonical_path: CanonicalPath,
    identifier: String,
    kind: ItemKind,
}

impl IndexedItem {
    pub(crate) fn new(
        kind: ItemKind,
        identifier: String,
        canonical_path: CanonicalPath,
        attributes: Vec<Attribute>,
    ) -> Self {
        Self {
            attributes,
            canonical_path,
            identifier,
            kind,
        }
    }

    pub fn attributes(&self) -> &[Attribute] {
        &self.attributes
    }

    pub fn canonical_path(&self) -> &CanonicalPath {
        &self.canonical_path
    }

    pub fn identifier(&self) -> &str {
        &self.identifier
    }

    pub fn kind(&self) -> ItemKind {
        self.kind
    }
}
