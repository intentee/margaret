use syn::Attribute;

use crate::canonical_path::CanonicalPath;
use crate::indexed_associated_type::IndexedAssociatedType;
use crate::indexed_attribute::IndexedAttribute;
use crate::indexed_method::IndexedMethod;
use crate::item_kind::ItemKind;

pub struct IndexedItem {
    associated_types: Vec<IndexedAssociatedType>,
    attributes: Vec<IndexedAttribute>,
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
            attributes: attributes.into_iter().map(IndexedAttribute::new).collect(),
            canonical_path,
            identifier,
            kind,
            methods: Vec::new(),
        }
    }

    #[must_use]
    pub fn associated_types(&self) -> &[IndexedAssociatedType] {
        &self.associated_types
    }

    #[must_use]
    pub fn attributes(&self) -> &[IndexedAttribute] {
        &self.attributes
    }

    #[must_use]
    pub fn canonical_path(&self) -> &CanonicalPath {
        &self.canonical_path
    }

    #[must_use]
    pub fn identifier(&self) -> &str {
        &self.identifier
    }

    #[must_use]
    pub fn kind(&self) -> ItemKind {
        self.kind
    }

    #[must_use]
    pub fn methods(&self) -> &[IndexedMethod] {
        &self.methods
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
}
