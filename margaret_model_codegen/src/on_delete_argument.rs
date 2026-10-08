use syn::Path;

use margaret_attributes::attribute_index::AttributeIndex;
use margaret_attributes::indexed_item::IndexedItem;
use margaret_model::on_delete::OnDelete;

use crate::on_delete_actions::ON_DELETE_ACTIONS;

pub(crate) enum OnDeleteArgument {
    Known(OnDelete),
    Unknown(Path),
}

impl OnDeleteArgument {
    pub(crate) fn of(index: &AttributeIndex, item: &IndexedItem, declared: Option<Path>) -> Self {
        match declared {
            None => Self::Known(OnDelete::NoAction),
            Some(path) => match index
                .resolve_item_path(item, &path)
                .as_ref()
                .and_then(|resolved| ON_DELETE_ACTIONS.variant(resolved))
            {
                Some(on_delete) => Self::Known(on_delete),
                None => Self::Unknown(path),
            },
        }
    }
}
