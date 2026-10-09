use syn::Attribute;
use syn::Path;

use crate::attribute_error::AttributeError;
use crate::attribute_host::AttributeHost;
use crate::canonical_path::CanonicalPath;
use crate::indexed_item::IndexedItem;
use crate::indexed_item_parts::IndexedItemParts;
use crate::indexed_trait_impl::IndexedTraitImpl;
use crate::indexed_variant::IndexedVariant;
use crate::item_is_copy::item_is_copy;
use crate::item_kind::ItemKind;
use crate::scanned_attribute::ScannedAttribute;
use crate::scanned_field::ScannedField;
use crate::scanned_method::ScannedMethod;

pub(crate) struct ScannedItem {
    attributes: Vec<ScannedAttribute>,
    canonical_path: CanonicalPath,
    fields: Vec<ScannedField>,
    identifier: String,
    kind: ItemKind,
    methods: Vec<ScannedMethod>,
    trait_impls: Vec<IndexedTraitImpl>,
    variants: Vec<IndexedVariant>,
}

impl ScannedItem {
    pub(crate) fn new(
        kind: ItemKind,
        identifier: String,
        canonical_path: CanonicalPath,
        attributes: Vec<Attribute>,
        fields: Vec<ScannedField>,
        variants: Vec<IndexedVariant>,
    ) -> Self {
        Self {
            attributes: ScannedAttribute::scan_all(attributes),
            canonical_path,
            fields,
            identifier,
            kind,
            methods: Vec::new(),
            trait_impls: Vec::new(),
            variants,
        }
    }

    pub(crate) fn add_method(&mut self, method: ScannedMethod) {
        self.methods.push(method);
    }

    pub(crate) fn add_trait_impl(&mut self, trait_impl: IndexedTraitImpl) {
        self.trait_impls.push(trait_impl);
    }

    pub(crate) fn canonical_path(&self) -> &CanonicalPath {
        &self.canonical_path
    }

    pub(crate) fn resolve(
        mut self,
        resolve: impl Copy + Fn(&[String], &Path) -> CanonicalPath,
    ) -> Result<IndexedItem, AttributeError> {
        self.methods
            .sort_by(|left, right| left.identifier().cmp(right.identifier()));
        let module_path = self
            .canonical_path
            .segments()
            .split_last()
            .map_or(&[][..], |(_, module)| module);
        let resolve_item_path = |path: &Path| resolve(module_path, path);
        let owner = &self.canonical_path;
        let attributes = ScannedAttribute::resolve_all(
            self.attributes,
            AttributeHost::Item,
            || owner.to_string(),
            resolve_item_path,
        )?;
        let fields = self
            .fields
            .into_iter()
            .map(|field| field.resolve(owner, resolve_item_path))
            .collect::<Result<Vec<_>, _>>()?;
        let methods = self
            .methods
            .into_iter()
            .map(|method| method.resolve(owner, resolve))
            .collect::<Result<Vec<_>, _>>()?;

        let is_copy = item_is_copy(&attributes, module_path, &self.trait_impls, resolve)?;

        Ok(IndexedItem::from_parts(IndexedItemParts {
            attributes,
            canonical_path: self.canonical_path,
            fields,
            identifier: self.identifier,
            is_copy,
            kind: self.kind,
            methods,
            trait_impls: self.trait_impls,
            variants: self.variants,
        }))
    }
}
