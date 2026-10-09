use uuid::Uuid;

pub(crate) struct FixtureRefreshToken {
    pub(crate) current: bool,
    pub(crate) family: Uuid,
}
