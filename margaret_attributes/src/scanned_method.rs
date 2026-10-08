use syn::Attribute;
use syn::FnArg;
use syn::Path;
use syn::Signature;

use crate::attribute_error::AttributeError;
use crate::attribute_host::AttributeHost;
use crate::canonical_path::CanonicalPath;
use crate::indexed_method::IndexedMethod;
use crate::scanned_attribute::ScannedAttribute;
use crate::scanned_parameter::ScannedParameter;

pub(crate) struct ScannedMethod {
    attributes: Vec<ScannedAttribute>,
    identifier: String,
    module_path: Vec<String>,
    parameters: Vec<ScannedParameter>,
    signature: Box<Signature>,
}

impl ScannedMethod {
    pub(crate) fn new(
        identifier: String,
        attributes: Vec<Attribute>,
        signature: Signature,
        module_path: Vec<String>,
    ) -> Self {
        let parameters = signature
            .inputs
            .iter()
            .enumerate()
            .filter_map(|(position, input)| match input {
                FnArg::Receiver(_) => None,
                FnArg::Typed(parameter) => Some(ScannedParameter::new(
                    parameter.attrs.clone(),
                    (*parameter.ty).clone(),
                    &parameter.pat,
                    position,
                )),
            })
            .collect();

        Self {
            attributes: ScannedAttribute::scan_all(attributes),
            identifier,
            module_path,
            parameters,
            signature: Box::new(signature),
        }
    }

    pub(crate) fn identifier(&self) -> &str {
        &self.identifier
    }

    pub(crate) fn resolve(
        self,
        owner: &CanonicalPath,
        resolve: impl Copy + Fn(&[String], &Path) -> CanonicalPath,
    ) -> Result<IndexedMethod, AttributeError> {
        let resolve_path = |path: &Path| resolve(&self.module_path, path);
        let method = format!("{owner}::{}", self.identifier);
        let attributes = ScannedAttribute::resolve_all(
            self.attributes,
            AttributeHost::Member,
            || method.clone(),
            resolve_path,
        )?;
        let parameters = self
            .parameters
            .into_iter()
            .map(|parameter| parameter.resolve(&method, resolve_path))
            .collect::<Result<Vec<_>, _>>()?;

        Ok(IndexedMethod::from_parts(
            attributes,
            self.identifier,
            parameters,
            self.signature,
        ))
    }
}
