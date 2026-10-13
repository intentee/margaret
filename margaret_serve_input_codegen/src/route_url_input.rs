#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct RouteUrlInput {
    pub path: String,
    pub server: String,
}
