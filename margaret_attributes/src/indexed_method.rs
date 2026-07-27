use syn::Attribute;
use syn::FnArg;
use syn::Signature;

use crate::attribute_selector::AttributeSelector;
use crate::indexed_attribute::IndexedAttribute;
use crate::indexed_parameter::IndexedParameter;

pub struct IndexedMethod {
    attributes: Vec<IndexedAttribute>,
    identifier: String,
    parameters: Vec<IndexedParameter>,
    signature: Box<Signature>,
}

impl IndexedMethod {
    pub fn new(identifier: String, attributes: Vec<Attribute>, signature: Signature) -> Self {
        let parameters = signature
            .inputs
            .iter()
            .enumerate()
            .filter_map(|(position, input)| match input {
                FnArg::Receiver(_) => None,
                FnArg::Typed(parameter) => Some(IndexedParameter::new(
                    parameter.attrs.clone(),
                    (*parameter.ty).clone(),
                    &parameter.pat,
                    position,
                )),
            })
            .collect();

        Self {
            attributes: attributes.into_iter().map(IndexedAttribute::new).collect(),
            identifier,
            parameters,
            signature: Box::new(signature),
        }
    }

    #[must_use]
    pub fn attributes(&self) -> &[IndexedAttribute] {
        &self.attributes
    }

    #[must_use]
    pub fn identifier(&self) -> &str {
        &self.identifier
    }

    #[must_use]
    pub fn has_attribute(&self, selector: &AttributeSelector) -> bool {
        self.attributes
            .iter()
            .any(|attribute| selector.matches(attribute.path()))
    }

    #[must_use]
    pub fn signature(&self) -> &Signature {
        self.signature.as_ref()
    }

    #[must_use]
    pub fn parameters(&self) -> &[IndexedParameter] {
        &self.parameters
    }

    #[must_use]
    pub fn has_receiver(&self) -> bool {
        self.signature.receiver().is_some()
    }
}
