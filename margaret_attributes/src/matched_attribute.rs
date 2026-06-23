use syn::Attribute;

use crate::attribute_args::AttributeArgs;
use crate::error::AttributeError;
use crate::indexed_item::IndexedItem;
use crate::path_text::format_path;

pub struct MatchedAttribute<'index> {
    attribute: &'index Attribute,
    item: &'index IndexedItem,
}

impl<'index> MatchedAttribute<'index> {
    pub(crate) fn new(item: &'index IndexedItem, attribute: &'index Attribute) -> Self {
        Self { attribute, item }
    }

    pub fn item(&self) -> &'index IndexedItem {
        self.item
    }

    pub fn path(&self) -> String {
        format_path(self.attribute.path())
    }

    pub fn args(&self) -> Result<AttributeArgs, AttributeError> {
        AttributeArgs::from_attribute(self.attribute)
    }
}
