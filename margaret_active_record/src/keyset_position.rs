use crate::page_cursor::PageCursor;

pub enum KeysetPosition<Modeled, Ordering> {
    From(PageCursor<Modeled, Ordering>),
    Start,
}

impl<Modeled, Ordering> Clone for KeysetPosition<Modeled, Ordering> {
    fn clone(&self) -> Self {
        match self {
            Self::From(cursor) => Self::From(cursor.clone()),
            Self::Start => Self::Start,
        }
    }
}
