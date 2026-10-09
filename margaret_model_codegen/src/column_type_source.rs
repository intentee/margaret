use syn::Type;

use margaret_attributes::attribute_index::AttributeIndex;
use margaret_attributes::indexed_item::IndexedItem;

use crate::decimal_canonical_path::DECIMAL_CANONICAL_PATH;
use crate::inferred_column::InferredColumn;
use crate::known_column_type::known_column_type;

pub(crate) enum ColumnTypeSource<'index> {
    Decimal,
    Known(InferredColumn),
    LocalEnum(&'index IndexedItem),
    LocalItem(&'index IndexedItem),
    Uninferrable,
}

impl<'index> ColumnTypeSource<'index> {
    pub(crate) fn of(
        attribute_index: &'index AttributeIndex,
        item: &IndexedItem,
        base: &Type,
    ) -> Self {
        let Some(path) = attribute_index.resolve_item_type(item, base) else {
            return ColumnTypeSource::Uninferrable;
        };

        if path == *DECIMAL_CANONICAL_PATH {
            return ColumnTypeSource::Decimal;
        }

        if let Some(indexed) = attribute_index.item(&path) {
            return if indexed.kind().is_enum() {
                ColumnTypeSource::LocalEnum(indexed)
            } else {
                ColumnTypeSource::LocalItem(indexed)
            };
        }

        match known_column_type(attribute_index, item, base, &path) {
            Some(known) => ColumnTypeSource::Known(known),
            None => ColumnTypeSource::Uninferrable,
        }
    }
}
