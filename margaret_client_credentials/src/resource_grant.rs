#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ResourceGrant {
    pub audience: &'static str,
    pub scopes: &'static [&'static str],
}
