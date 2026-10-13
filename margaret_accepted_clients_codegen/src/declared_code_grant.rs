use crate::declared_code_policy::DeclaredCodePolicy;

#[derive(Debug)]
pub enum DeclaredCodeGrant {
    Granted(DeclaredCodePolicy),
    Withheld,
}
