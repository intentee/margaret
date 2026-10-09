use std::fmt;
use std::fmt::Display;
use std::fmt::Formatter;

use uuid::Uuid;

pub struct AuthorNotFound {
    pub author_id: Uuid,
}

impl Display for AuthorNotFound {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        write!(formatter, "no author exists with id '{}'", self.author_id)
    }
}
