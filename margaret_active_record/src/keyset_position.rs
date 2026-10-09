use crate::page_cursor::PageCursor;

pub enum KeysetPosition<Modeled, Ordering, Toward> {
    From(PageCursor<Modeled, Ordering, Toward>),
    Start,
}

impl<Modeled, Ordering, Toward> Clone for KeysetPosition<Modeled, Ordering, Toward> {
    fn clone(&self) -> Self {
        match self {
            Self::From(cursor) => Self::From(cursor.clone()),
            Self::Start => Self::Start,
        }
    }
}
