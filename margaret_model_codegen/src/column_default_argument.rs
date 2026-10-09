use syn::Path;

use margaret_attributes::attribute_index::AttributeIndex;
use margaret_attributes::indexed_item::IndexedItem;
use margaret_model::column_default::ColumnDefault;

use crate::column_defaults::COLUMN_DEFAULTS;

pub(crate) enum ColumnDefaultArgument {
    Known(ColumnDefault),
    Missing,
    Unknown(Path),
}

impl ColumnDefaultArgument {
    pub(crate) fn of(index: &AttributeIndex, item: &IndexedItem, declared: Option<Path>) -> Self {
        match declared {
            None => Self::Missing,
            Some(path) => match index
                .resolve_item_path(item, &path)
                .as_ref()
                .and_then(|resolved| COLUMN_DEFAULTS.variant(resolved))
            {
                Some(column_default) => Self::Known(column_default),
                None => Self::Unknown(path),
            },
        }
    }
}
