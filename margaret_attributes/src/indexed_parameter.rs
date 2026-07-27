use proc_macro2::Ident;
use quote::format_ident;
use syn::Attribute;
use syn::Pat;
use syn::Type;

use crate::canonical_path::CanonicalPath;
use crate::framework_attribute::FrameworkAttribute;
use crate::indexed_attribute::IndexedAttribute;

pub struct IndexedParameter {
    attributes: Vec<IndexedAttribute>,
    declared: Type,
    diagnostic_name: String,
    holder: Ident,
    position: usize,
}

impl IndexedParameter {
    pub(crate) fn new(
        attributes: Vec<Attribute>,
        declared: Type,
        pattern: &Pat,
        position: usize,
    ) -> Self {
        let (diagnostic_name, holder) = match pattern {
            Pat::Ident(pattern_ident) => {
                (pattern_ident.ident.to_string(), pattern_ident.ident.clone())
            }
            _ => (position.to_string(), format_ident!("argument_{position}")),
        };

        Self {
            attributes: attributes.into_iter().map(IndexedAttribute::new).collect(),
            declared,
            diagnostic_name,
            holder,
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
    pub fn diagnostic_name(&self) -> &str {
        &self.diagnostic_name
    }

    #[must_use]
    pub fn framework_attribute(&self, attribute: FrameworkAttribute) -> Option<&IndexedAttribute> {
        self.attributes
            .iter()
            .find(|indexed| indexed.framework_attribute() == Some(attribute))
    }

    #[must_use]
    pub fn holder(&self) -> &Ident {
        &self.holder
    }

    #[must_use]
    pub fn position(&self) -> usize {
        self.position
    }

    pub(crate) fn resolve_attribute_paths(
        &mut self,
        resolve: impl Fn(&syn::Path) -> CanonicalPath,
    ) {
        for attribute in &mut self.attributes {
            attribute.set_canonical_path(resolve(attribute.path()));
        }
    }
}
