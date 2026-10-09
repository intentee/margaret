use syn::Type;
use syn::TypePath;

use margaret_attributes::attribute_index::AttributeIndex;
use margaret_attributes::canonical_path::CanonicalPath;
use margaret_attributes::indexed_item::IndexedItem;

fn plain_type_path(written: &Type) -> Option<&TypePath> {
    let Type::Path(type_path) = written else {
        return None;
    };

    (type_path.qself.is_none()
        && type_path
            .path
            .segments
            .iter()
            .all(|segment| segment.arguments.is_none()))
    .then_some(type_path)
}

pub(crate) enum PlainTypeResolution {
    NotPlain,
    Resolved(CanonicalPath),
    Unknown,
}

impl PlainTypeResolution {
    pub(crate) fn of(index: &AttributeIndex, item: &IndexedItem, written: &Type) -> Self {
        match plain_type_path(written) {
            None => Self::NotPlain,
            Some(type_path) => match index.resolve_item_path(item, &type_path.path) {
                None => Self::Unknown,
                Some(resolved) => Self::Resolved(resolved),
            },
        }
    }
}
