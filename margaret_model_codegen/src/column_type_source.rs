use syn::Type;

use margaret_attributes::attribute_index::AttributeIndex;
use margaret_attributes::indexed_item::IndexedItem;

use crate::decimal_canonical_path::DECIMAL_CANONICAL_PATH;

pub(crate) enum ColumnTypeSource<'index> {
    DeclaredType,
    Decimal,
    LocalEnum(&'index IndexedItem),
    LocalItem(&'index IndexedItem),
}

impl<'index> ColumnTypeSource<'index> {
    pub(crate) fn of(
        attribute_index: &'index AttributeIndex,
        item: &IndexedItem,
        base: &Type,
    ) -> Self {
        let Some(path) = attribute_index.resolve_item_type(item, base) else {
            return ColumnTypeSource::DeclaredType;
        };

        if path == *DECIMAL_CANONICAL_PATH {
            return ColumnTypeSource::Decimal;
        }

        match attribute_index.item(&path) {
            None => ColumnTypeSource::DeclaredType,
            Some(indexed) if indexed.kind().is_enum() => ColumnTypeSource::LocalEnum(indexed),
            Some(indexed) => ColumnTypeSource::LocalItem(indexed),
        }
    }
}
