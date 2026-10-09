use margaret_model::table::Table;

use crate::field_columns::field_columns;
use crate::field_span::FieldSpan;

pub(crate) fn ordering_columns(
    table: &'static Table,
    head: Vec<&'static str>,
    rest: &[FieldSpan],
) -> Vec<&'static str> {
    head.into_iter()
        .chain(
            rest.iter()
                .flat_map(|span| field_columns(table, span.start, span.width)),
        )
        .collect()
}
