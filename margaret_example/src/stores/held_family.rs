use margaret::framework::authorization_grants::refresh_family::RefreshFamily;

#[derive(Clone)]
pub enum HeldFamily {
    Open(RefreshFamily),
    Revoked,
}
