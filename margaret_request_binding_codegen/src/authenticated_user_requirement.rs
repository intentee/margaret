#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AuthenticatedUserRequirement {
    Optional,
    Required,
}
