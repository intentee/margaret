#[cfg(test)]
use syn::Attribute;

use crate::canonical_path::CanonicalPath;
use crate::framework_attribute::FrameworkAttribute;
use crate::indexed_attribute::IndexedAttribute;
use crate::indexed_field::IndexedField;
use crate::indexed_item_parts::IndexedItemParts;
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
    #[cfg(test)]
    pub(crate) fn new(
        kind: ItemKind,
        identifier: String,
        canonical_path: CanonicalPath,
        attributes: Vec<Attribute>,
        fields: Vec<IndexedField>,
        variants: Vec<IndexedVariant>,
    ) -> Self {
        Self {
            attributes: attributes
                .into_iter()
                .map(|attribute| IndexedAttribute::new(&attribute))
                .collect(),
            canonical_path,
            fields,
            identifier,
            kind,
            methods: Vec::new(),
            trait_impls: Vec::new(),
            variants,
        }
    }

    pub(crate) fn from_parts(parts: IndexedItemParts) -> Self {
        let IndexedItemParts {
            attributes,
            canonical_path,
            fields,
            identifier,
            kind,
            methods,
            trait_impls,
            variants,
        } = parts;

        Self {
            attributes,
            canonical_path,
            fields,
            identifier,
            kind,
            methods,
            trait_impls,
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
}
