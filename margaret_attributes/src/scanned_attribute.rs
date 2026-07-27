use syn::Attribute;

use crate::canonical_path::CanonicalPath;
use crate::indexed_attribute::IndexedAttribute;

pub(crate) struct ScannedAttribute {
    attribute: Attribute,
}

impl ScannedAttribute {
    pub(crate) fn scan_all(attributes: Vec<Attribute>) -> Vec<Self> {
        attributes.into_iter().map(Self::new).collect()
    }

    pub(crate) fn resolve_all(
        attributes: Vec<Self>,
        resolve: impl Copy + Fn(&syn::Path) -> CanonicalPath,
    ) -> Vec<IndexedAttribute> {
        attributes
            .into_iter()
            .map(|attribute| {
                let canonical_path = resolve(attribute.attribute.path());

                IndexedAttribute::from_canonical(attribute.attribute, canonical_path)
            })
            .collect()
    }

    fn new(attribute: Attribute) -> Self {
        Self { attribute }
    }
}
