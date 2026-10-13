use crate::field_columns::field_columns;
use crate::record::Record;

pub(crate) fn primary_key_columns<Keyed: Record>() -> Vec<&'static str> {
    Keyed::PRIMARY_KEY
        .iter()
        .flat_map(|span| field_columns(Keyed::TABLE, span.start, span.width))
        .collect()
}
