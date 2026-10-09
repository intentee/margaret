use crate::page_cursor::PageCursor;

pub enum NextPage<Modeled, Ordering, Toward> {
    Continues(PageCursor<Modeled, Ordering, Toward>),
    Exhausted,
}
