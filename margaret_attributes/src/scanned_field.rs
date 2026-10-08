use syn::Attribute;
use syn::Path;
use syn::Type;

use crate::attribute_error::AttributeError;
use crate::attribute_host::AttributeHost;
use crate::canonical_path::CanonicalPath;
use crate::field_identifier::FieldIdentifier;
use crate::indexed_field::IndexedField;
use crate::scanned_attribute::ScannedAttribute;

pub(crate) struct ScannedField {
    attributes: Vec<ScannedAttribute>,
    identifier: FieldIdentifier,
    ty: Type,
}

impl ScannedField {
    pub(crate) fn new(identifier: FieldIdentifier, ty: Type, attributes: Vec<Attribute>) -> Self {
        Self {
            attributes: ScannedAttribute::scan_all(attributes),
            identifier,
            ty,
        }
    }

    pub(crate) fn resolve(
        self,
        owner: &CanonicalPath,
        resolve: impl Copy + Fn(&Path) -> CanonicalPath,
    ) -> Result<IndexedField, AttributeError> {
        let attributes = ScannedAttribute::resolve_all(
            self.attributes,
            AttributeHost::Member,
            || format!("{owner}::{}", self.identifier),
            resolve,
        )?;

        Ok(IndexedField::from_parts(
            self.identifier,
            self.ty,
            attributes,
        ))
    }
}
