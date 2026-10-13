use syn::Type;
use syn::TypePath;

use margaret_attributes::attribute_index::AttributeIndex;
use margaret_attributes::canonical_path::CanonicalPath;
use margaret_attributes::indexed_item::IndexedItem;
use margaret_attributes::standard_library_item::StandardLibraryItem;
use margaret_syn_type_peeling::single_generic_argument::single_generic_argument;

use crate::children_canonical_path::CHILDREN_CANONICAL_PATH;
use crate::loaded_wrapper::LoadedWrapper;

fn owned_path(declared: &Type) -> Option<&TypePath> {
    match declared {
        Type::Path(type_path) => Some(type_path),
        _ => None,
    }
}

pub(crate) struct LoadedType {
    pub(crate) loaded: CanonicalPath,
    pub(crate) wrapper: LoadedWrapper,
}

impl LoadedType {
    pub(crate) fn of(index: &AttributeIndex, item: &IndexedItem, declared: &Type) -> Option<Self> {
        let type_path = owned_path(declared)?;
        let outer = index.resolve_item_path(item, &type_path.path)?;
        let wrapper =
            if StandardLibraryItem::from_canonical(&outer) == Some(StandardLibraryItem::Option) {
                LoadedWrapper::Optional
            } else if outer == *CHILDREN_CANONICAL_PATH {
                LoadedWrapper::Children
            } else {
                return Some(Self {
                    loaded: outer,
                    wrapper: LoadedWrapper::Bare,
                });
            };
        let argument = type_path
            .path
            .segments
            .last()
            .and_then(single_generic_argument)?;
        let loaded = index.resolve_item_path(item, &owned_path(argument)?.path)?;

        Some(Self { loaded, wrapper })
    }
}
