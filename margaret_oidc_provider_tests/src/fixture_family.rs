use margaret_authorization_grants::refresh_family::RefreshFamily;

pub(crate) enum FixtureFamily {
    Open(RefreshFamily),
    Revoked,
}
