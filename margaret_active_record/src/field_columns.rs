use margaret_model::table::Table;

pub(crate) fn field_columns(
    table: &'static Table,
    start: usize,
    width: usize,
) -> Vec<&'static str> {
    table
        .columns
        .iter()
        .skip(start)
        .take(width)
        .map(|column| column.name)
        .collect()
}
