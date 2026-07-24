use syn::Attribute;

use crate::attribute_selector::AttributeSelector;
use crate::canonical_path::CanonicalPath;
use crate::indexed_attribute::IndexedAttribute;
use crate::indexed_field::IndexedField;
use crate::indexed_method::IndexedMethod;
use crate::indexed_trait_impl::IndexedTraitImpl;
use crate::item_kind::ItemKind;

pub struct IndexedItem {
    attributes: Vec<IndexedAttribute>,
    canonical_path: CanonicalPath,
    fields: Vec<IndexedField>,
    identifier: String,
    kind: ItemKind,
    methods: Vec<IndexedMethod>,
    trait_impls: Vec<IndexedTraitImpl>,
}

impl IndexedItem {
    pub(crate) fn new(
        kind: ItemKind,
        identifier: String,
        canonical_path: CanonicalPath,
        attributes: Vec<Attribute>,
        fields: Vec<IndexedField>,
    ) -> Self {
        Self {
            attributes: attributes.into_iter().map(IndexedAttribute::new).collect(),
            canonical_path,
            fields,
            identifier,
            kind,
            methods: Vec::new(),
            trait_impls: Vec::new(),
        }
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
    pub fn fields(&self) -> &[IndexedField] {
        &self.fields
    }

    #[must_use]
    pub fn has_attribute(&self, selector: &AttributeSelector) -> bool {
        self.attributes
            .iter()
            .any(|attribute| selector.matches(attribute.path()))
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

    #[must_use]
    pub fn trait_impls(&self) -> &[IndexedTraitImpl] {
        &self.trait_impls
    }

    pub(crate) fn add_method(&mut self, method: IndexedMethod) {
        self.methods.push(method);
    }

    pub(crate) fn add_trait_impl(&mut self, trait_impl: IndexedTraitImpl) {
        self.trait_impls.push(trait_impl);
    }

    pub(crate) fn sort_members(&mut self) {
        self.methods
            .sort_by(|left, right| left.identifier().cmp(right.identifier()));
    }
}
