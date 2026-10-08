use crate::page_cursor::PageCursor;

pub enum KeysetPosition<Modeled> {
    From(PageCursor<Modeled>),
    Start,
}

impl<Modeled> Clone for KeysetPosition<Modeled> {
    fn clone(&self) -> Self {
        match self {
            Self::From(cursor) => Self::From(cursor.clone()),
            Self::Start => Self::Start,
        }
    }
}
