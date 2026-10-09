use crate::page_cursor::PageCursor;

pub enum NextPage<Modeled, Ordering> {
    Continues(PageCursor<Modeled, Ordering>),
    Exhausted,
}
