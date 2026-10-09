use proc_macro2::Ident;
use quote::format_ident;
use syn::Attribute;
use syn::Pat;
use syn::Type;

use crate::framework_attribute::FrameworkAttribute;
use crate::indexed_attribute::IndexedAttribute;

pub struct IndexedParameter {
    attributes: Vec<IndexedAttribute>,
    declared: Type,
    name: Ident,
    position: usize,
}

impl IndexedParameter {
    pub(crate) fn from_parts(
        attributes: Vec<IndexedAttribute>,
        declared: Type,
        name: Ident,
        position: usize,
    ) -> Self {
        Self {
            attributes,
            declared,
            name,
            position,
        }
    }

    pub(crate) fn new(
        attributes: Vec<Attribute>,
        declared: Type,
        pattern: &Pat,
        position: usize,
    ) -> Self {
        let name = match pattern {
            Pat::Ident(pattern_ident) => pattern_ident.ident.clone(),
            _ => format_ident!("argument_{position}"),
        };

        Self {
            attributes: attributes
                .into_iter()
                .map(|attribute| IndexedAttribute::new(&attribute))
                .collect(),
            declared,
            name,
            position,
        }
    }

    #[must_use]
    pub fn attributes(&self) -> &[IndexedAttribute] {
        &self.attributes
    }

    #[must_use]
    pub fn declared(&self) -> &Type {
        &self.declared
    }

    #[must_use]
    pub fn framework_attribute(&self, attribute: FrameworkAttribute) -> Option<&IndexedAttribute> {
        self.attributes
            .iter()
            .find(|indexed| indexed.framework_attribute() == Some(attribute))
    }

    #[must_use]
    pub fn name(&self) -> &Ident {
        &self.name
    }

    #[must_use]
    pub fn position(&self) -> usize {
        self.position
    }
}
