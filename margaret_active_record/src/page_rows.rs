use tokio_postgres::Row;

use margaret_model::table::Table;

use crate::active_record_error::ActiveRecordError;
use crate::next_page::NextPage;
use crate::page_cursor::PageCursor;
use crate::raw_column::RawColumn;
use crate::row_column::row_column;

fn cursor_of<Modeled, Ordering>(
    row: &Row,
    offset: usize,
    width: usize,
    table: &'static Table,
) -> Result<PageCursor<Modeled, Ordering>, ActiveRecordError> {
    (offset..offset + width)
        .map(|position| row_column::<RawColumn>(row, position, table))
        .collect::<Result<Vec<RawColumn>, ActiveRecordError>>()
        .map(PageCursor::new)
}

pub(crate) fn next_page<Modeled, Ordering>(
    rows: &[Row],
    limit: usize,
    cursor_offset: usize,
    cursor_width: usize,
    table: &'static Table,
) -> Result<NextPage<Modeled, Ordering>, ActiveRecordError> {
    match rows.get(limit) {
        Some(overflow) => {
            cursor_of(overflow, cursor_offset, cursor_width, table).map(NextPage::Continues)
        }
        None => Ok(NextPage::Exhausted),
    }
}
