use syn::Attribute;
use syn::Signature;

pub struct IndexedMethod {
    attributes: Vec<Attribute>,
    identifier: String,
    signature: Box<Signature>,
}

impl IndexedMethod {
    pub(crate) fn new(
        identifier: String,
        attributes: Vec<Attribute>,
        signature: Signature,
    ) -> Self {
        Self {
            attributes,
            identifier,
            signature: Box::new(signature),
        }
    }

    #[must_use]
    pub fn attributes(&self) -> &[Attribute] {
        &self.attributes
    }

    #[must_use]
    pub fn identifier(&self) -> &str {
        &self.identifier
    }

    #[must_use]
    pub fn signature(&self) -> &Signature {
        self.signature.as_ref()
    }
}
