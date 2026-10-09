use margaret_attributes::tag::Tag;

use crate::session_source::SessionSource;

pub(crate) enum SessionIssuance<'declarations> {
    Declared {
        source: SessionSource,
        tag: &'declarations Tag,
    },
    Undeclared,
}
