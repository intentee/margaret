use syn::Attribute;

use crate::canonical_path::CanonicalPath;
use crate::framework_attribute::FrameworkAttribute;
use crate::indexed_attribute::IndexedAttribute;
use crate::indexed_field::IndexedField;
use crate::indexed_method::IndexedMethod;
use crate::indexed_trait_impl::IndexedTraitImpl;
use crate::indexed_variant::IndexedVariant;
use crate::item_kind::ItemKind;

pub struct IndexedItem {
    attributes: Vec<IndexedAttribute>,
    canonical_path: CanonicalPath,
    fields: Vec<IndexedField>,
    identifier: String,
    kind: ItemKind,
    methods: Vec<IndexedMethod>,
    trait_impls: Vec<IndexedTraitImpl>,
    variants: Vec<IndexedVariant>,
}

impl IndexedItem {
    pub(crate) fn new(
        kind: ItemKind,
        identifier: String,
        canonical_path: CanonicalPath,
        attributes: Vec<Attribute>,
        fields: Vec<IndexedField>,
        variants: Vec<IndexedVariant>,
    ) -> Self {
        Self {
            attributes: attributes.into_iter().map(IndexedAttribute::new).collect(),
            canonical_path,
            fields,
            identifier,
            kind,
            methods: Vec::new(),
            trait_impls: Vec::new(),
            variants,
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
    pub fn has_framework_attribute(&self, attribute: FrameworkAttribute) -> bool {
        self.attributes
            .iter()
            .any(|indexed| indexed.framework_attribute() == Some(attribute))
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

    #[must_use]
    pub fn variants(&self) -> &[IndexedVariant] {
        &self.variants
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

    pub(crate) fn resolve_attribute_paths(
        &mut self,
        resolve: impl Copy + Fn(&syn::Path) -> CanonicalPath,
    ) {
        for attribute in &mut self.attributes {
            attribute.set_canonical_path(resolve(attribute.path()));
        }

        for field in &mut self.fields {
            field.resolve_attribute_paths(resolve);
        }

        for method in &mut self.methods {
            method.resolve_attribute_paths(resolve);
        }
    }
}
