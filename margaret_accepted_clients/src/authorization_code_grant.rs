use crate::code_grant_policy::CodeGrantPolicy;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AuthorizationCodeGrant {
    Granted(CodeGrantPolicy),
    Withheld,
}
