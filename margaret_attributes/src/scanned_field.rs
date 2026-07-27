use syn::Attribute;
use syn::Type;

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
        resolve: impl Copy + Fn(&syn::Path) -> CanonicalPath,
    ) -> IndexedField {
        IndexedField::from_parts(
            self.identifier,
            self.ty,
            ScannedAttribute::resolve_all(self.attributes, resolve),
        )
    }
}
