use syn::Type;

use margaret_attributes::attribute_index::AttributeIndex;
use margaret_attributes::indexed_item::IndexedItem;
use margaret_attributes::standard_library_item::StandardLibraryItem;
use margaret_syn_type_peeling::peel_standard_wrapper::peel_standard_wrapper;

pub struct OptionalParameter {
    pub required: bool,
    pub value_type: Type,
}

impl OptionalParameter {
    #[must_use]
    pub fn from_type(index: &AttributeIndex, item: &IndexedItem, declared: &Type) -> Self {
        match peel_standard_wrapper(index, item, declared, &[StandardLibraryItem::Option]) {
            Some(inner) => Self {
                required: false,
                value_type: inner.clone(),
            },
            None => Self {
                required: true,
                value_type: declared.clone(),
            },
        }
    }
}
