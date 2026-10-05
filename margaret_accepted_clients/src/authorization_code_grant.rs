use crate::code_grant_policy::CodeGrantPolicy;

pub enum AuthorizationCodeGrant {
    Granted(CodeGrantPolicy),
    Withheld,
}
