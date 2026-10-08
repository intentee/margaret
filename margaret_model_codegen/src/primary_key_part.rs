use crate::primary_key_source::PrimaryKeySource;

pub(crate) struct PrimaryKeyPart {
    pub(crate) field: String,
    pub(crate) source: PrimaryKeySource,
}
