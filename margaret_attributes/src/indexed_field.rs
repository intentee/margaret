use syn::Attribute;
use syn::Type;

use crate::field_identifier::FieldIdentifier;
use crate::indexed_attribute::IndexedAttribute;

pub struct IndexedField {
    attributes: Vec<IndexedAttribute>,
    identifier: FieldIdentifier,
    ty: Type,
}

impl IndexedField {
    pub(crate) fn new(identifier: FieldIdentifier, ty: Type, attributes: Vec<Attribute>) -> Self {
        Self {
            attributes: attributes.into_iter().map(IndexedAttribute::new).collect(),
            identifier,
            ty,
        }
    }

    #[must_use]
    pub fn attributes(&self) -> &[IndexedAttribute] {
        &self.attributes
    }

    #[must_use]
    pub fn identifier(&self) -> &FieldIdentifier {
        &self.identifier
    }

    #[must_use]
    pub fn ty(&self) -> &Type {
        &self.ty
    }
}

#[cfg(test)]
mod tests {
    use syn::Type;
    use syn::parse_quote;

    use crate::field_identifier::FieldIdentifier;
    use crate::indexed_field::IndexedField;

    #[test]
    fn exposes_the_identifier_type_and_attributes() {
        let expected_type: Type = parse_quote!(String);
        let field = IndexedField::new(
            FieldIdentifier::Named("title".to_string()),
            expected_type.clone(),
            vec![parse_quote!(#[column(name = "title")])],
        );

        assert_eq!(
            field.identifier(),
            &FieldIdentifier::Named("title".to_string())
        );
        assert_eq!(field.ty(), &expected_type);
        assert_eq!(field.attributes().len(), 1);
        assert_eq!(
            field.attributes()[0]
                .args()
                .expect("the column arguments parse")
                .string("name")
                .expect("the name is a string literal"),
            Some("title".to_string())
        );
    }
}
