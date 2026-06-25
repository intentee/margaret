use syn::Attribute;

use crate::canonical_path::CanonicalPath;
use crate::indexed_associated_type::IndexedAssociatedType;
use crate::indexed_method::IndexedMethod;
use crate::item_kind::ItemKind;

pub struct IndexedItem {
    associated_types: Vec<IndexedAssociatedType>,
    attributes: Vec<Attribute>,
    canonical_path: CanonicalPath,
    identifier: String,
    kind: ItemKind,
    methods: Vec<IndexedMethod>,
}

impl IndexedItem {
    pub(crate) fn new(
        kind: ItemKind,
        identifier: String,
        canonical_path: CanonicalPath,
        attributes: Vec<Attribute>,
    ) -> Self {
        Self {
            associated_types: Vec::new(),
            attributes,
            canonical_path,
            identifier,
            kind,
            methods: Vec::new(),
        }
    }

    pub(crate) fn add_associated_type(&mut self, associated_type: IndexedAssociatedType) {
        self.associated_types.push(associated_type);
    }

    pub(crate) fn add_method(&mut self, method: IndexedMethod) {
        self.methods.push(method);
    }

    pub(crate) fn sort_members(&mut self) {
        self.associated_types
            .sort_by(|left, right| left.name().cmp(right.name()));
        self.methods
            .sort_by(|left, right| left.identifier().cmp(right.identifier()));
    }

    pub fn associated_types(&self) -> &[IndexedAssociatedType] {
        &self.associated_types
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

    pub fn methods(&self) -> &[IndexedMethod] {
        &self.methods
    }
}
