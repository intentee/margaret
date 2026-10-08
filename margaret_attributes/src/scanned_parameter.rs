use proc_macro2::Ident;
use quote::format_ident;
use syn::Attribute;
use syn::Pat;
use syn::Path;
use syn::Type;

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
        resolve: impl Copy + Fn(&Path) -> CanonicalPath,
    ) -> IndexedParameter {
        IndexedParameter::from_parts(
            ScannedAttribute::resolve_all(self.attributes, resolve),
            self.declared,
            self.name,
            self.position,
        )
    }
}
