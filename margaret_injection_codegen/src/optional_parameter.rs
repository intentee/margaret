use syn::Type;

use margaret_attributes::attribute_index::AttributeIndex;
use margaret_attributes::standard_library_item::StandardLibraryItem;
use margaret_syn_type_peeling::peel_standard_wrapper::peel_standard_wrapper;

pub struct OptionalParameter {
    pub required: bool,
    pub value_type: Type,
}

impl OptionalParameter {
    #[must_use]
    pub fn from_type(index: &AttributeIndex, module_path: &[String], declared: &Type) -> Self {
        match peel_standard_wrapper(index, module_path, declared, &[StandardLibraryItem::Option]) {
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
