use syn::Type;

use margaret_attributes::attribute_index::AttributeIndex;
use margaret_attributes::canonical_path::CanonicalPath;
use margaret_attributes::indexed_item::IndexedItem;
use margaret_attributes::standard_library_item::StandardLibraryItem;
use margaret_model::column_type::ColumnType;
use margaret_syn_type_peeling::peel_standard_wrapper::peel_standard_wrapper;

use crate::date_time_canonical_path::DATE_TIME_CANONICAL_PATH;
use crate::inferred_column::InferredColumn;
use crate::uuid_canonical_path::UUID_CANONICAL_PATH;

fn column(column_type: ColumnType) -> InferredColumn {
    InferredColumn {
        column_type,
        nullable: false,
    }
}

fn is_byte_vector(attribute_index: &AttributeIndex, item: &IndexedItem, base: &Type) -> bool {
    peel_standard_wrapper(attribute_index, item, base, &[StandardLibraryItem::Vec])
        .and_then(|element| attribute_index.resolve_item_type(item, element))
        .is_some_and(|element| element.segments() == ["u8"])
}

fn primitive_column_type(primitive: &str) -> Option<ColumnType> {
    match primitive {
        "bool" => Some(ColumnType::Boolean),
        "f32" => Some(ColumnType::Real),
        "f64" => Some(ColumnType::DoublePrecision),
        "i32" => Some(ColumnType::Integer),
        "i64" => Some(ColumnType::BigInt),
        _ => None,
    }
}

pub(crate) fn known_column_type(
    attribute_index: &AttributeIndex,
    item: &IndexedItem,
    base: &Type,
    canonical: &CanonicalPath,
) -> Option<InferredColumn> {
    if *canonical == *UUID_CANONICAL_PATH {
        return Some(column(ColumnType::Uuid));
    }

    if *canonical == *DATE_TIME_CANONICAL_PATH {
        return Some(column(ColumnType::Timestamptz));
    }

    match StandardLibraryItem::from_canonical(canonical) {
        Some(StandardLibraryItem::String) => Some(column(ColumnType::Text)),
        Some(StandardLibraryItem::Vec) if is_byte_vector(attribute_index, item, base) => {
            Some(column(ColumnType::Bytea))
        }
        Some(_) => None,
        None => match canonical.segments() {
            [primitive] => primitive_column_type(primitive).map(column),
            _ => None,
        },
    }
}
