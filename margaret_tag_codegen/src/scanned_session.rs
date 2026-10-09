use margaret_attributes::indexed_attribute::IndexedAttribute;

use crate::session_source::SessionSource;

pub(crate) struct ScannedSession<'index> {
    pub(crate) attribute: &'index IndexedAttribute,
    pub(crate) source: SessionSource,
}
