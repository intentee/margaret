use syn::Attribute;
use syn::Signature;

use crate::canonical_path::CanonicalPath;

pub struct IndexedMethod {
    attributes: Vec<Attribute>,
    identifier: String,
    self_type_path: CanonicalPath,
    signature: Box<Signature>,
}

impl IndexedMethod {
    pub(crate) fn new(
        self_type_path: CanonicalPath,
        identifier: String,
        attributes: Vec<Attribute>,
        signature: Signature,
    ) -> Self {
        Self {
            attributes,
            identifier,
            self_type_path,
            signature: Box::new(signature),
        }
    }

    pub fn attributes(&self) -> &[Attribute] {
        &self.attributes
    }

    pub fn identifier(&self) -> &str {
        &self.identifier
    }

    pub fn self_type_path(&self) -> &CanonicalPath {
        &self.self_type_path
    }

    pub fn signature(&self) -> &Signature {
        self.signature.as_ref()
    }
}
