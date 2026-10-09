use syn::Type;

use margaret_attributes::attribute_index::AttributeIndex;
use margaret_attributes::indexed_item::IndexedItem;
use margaret_attributes::standard_library_item::StandardLibraryItem;
use margaret_syn_type_peeling::peel_standard_wrapper::peel_standard_wrapper;

pub(crate) struct DeclaredColumnType<'field> {
    pub(crate) base: &'field Type,
    pub(crate) declared: &'field Type,
    pub(crate) nullable: bool,
}

impl<'field> DeclaredColumnType<'field> {
    pub(crate) fn of(
        attribute_index: &AttributeIndex,
        item: &IndexedItem,
        declared: &'field Type,
    ) -> Self {
        match peel_standard_wrapper(
            attribute_index,
            item,
            declared,
            &[StandardLibraryItem::Option],
        ) {
            Some(base) => Self {
                base,
                declared,
                nullable: true,
            },
            None => Self {
                base: declared,
                declared,
                nullable: false,
            },
        }
    }
}
