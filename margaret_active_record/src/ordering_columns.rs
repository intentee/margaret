use margaret_model::table::Table;

use crate::field_columns::field_columns;
use crate::field_span::FieldSpan;

pub(crate) fn ordering_columns(table: &'static Table, spans: &[FieldSpan]) -> Vec<&'static str> {
    spans
        .iter()
        .flat_map(|span| field_columns(table, span.start, span.width))
        .collect()
}
