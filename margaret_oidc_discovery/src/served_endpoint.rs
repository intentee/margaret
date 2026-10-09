#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ServedEndpoint {
    Served(&'static str),
    Unserved,
}
