#[derive(Clone, Debug, Eq, PartialEq)]
pub enum DerivedEndpoint {
    Served(String),
    Unserved,
}
