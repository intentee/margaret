#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TokenRejection {
    Expired,
    Invalid,
}
