use proc_macro2::Ident;
use quote::format_ident;
use syn::Attribute;
use syn::Pat;
use syn::Path;
use syn::Type;

use crate::attribute_error::AttributeError;
use crate::attribute_host::AttributeHost;
use crate::canonical_path::CanonicalPath;
use crate::indexed_parameter::IndexedParameter;
use crate::scanned_attribute::ScannedAttribute;

pub(crate) struct ScannedParameter {
    attributes: Vec<ScannedAttribute>,
    declared: Type,
    name: Ident,
    position: usize,
}

impl ScannedParameter {
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
            attributes: ScannedAttribute::scan_all(attributes),
            declared,
            name,
            position,
        }
    }

    pub(crate) fn resolve(
        self,
        method: &str,
        resolve: impl Copy + Fn(&Path) -> CanonicalPath,
    ) -> Result<IndexedParameter, AttributeError> {
        let attributes = ScannedAttribute::resolve_all(
            self.attributes,
            AttributeHost::Member,
            || format!("argument #{} of '{method}'", self.position),
            resolve,
        )?;

        Ok(IndexedParameter::from_parts(
            attributes,
            self.declared,
            self.name,
            self.position,
        ))
    }
}
