#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AuthorizationResponseIssuer {
    Advertised,
    Unadvertised,
}
