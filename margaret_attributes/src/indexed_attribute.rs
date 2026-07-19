use std::cell::OnceCell;

use syn::Attribute;
use syn::Path;

use crate::attribute_args::AttributeArgs;
use crate::attribute_error::AttributeError;

pub struct IndexedAttribute {
    args_cache: OnceCell<AttributeArgs>,
    attribute: Attribute,
}

impl IndexedAttribute {
    pub(crate) fn new(attribute: Attribute) -> Self {
        Self {
            args_cache: OnceCell::new(),
            attribute,
        }
    }

    #[must_use]
    pub fn path(&self) -> &Path {
        self.attribute.path()
    }

    pub(crate) fn args(&self) -> Result<&AttributeArgs, AttributeError> {
        if let Some(cached) = self.args_cache.get() {
            return Ok(cached);
        }

        let parsed = AttributeArgs::from_attribute(&self.attribute)?;

        Ok(self.args_cache.get_or_init(|| parsed))
    }
}
