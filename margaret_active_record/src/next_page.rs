use crate::page_cursor::PageCursor;

pub enum NextPage<Modeled> {
    Continues(PageCursor<Modeled>),
    Exhausted,
}
