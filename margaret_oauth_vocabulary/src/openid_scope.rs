use crate::declared_scope::DeclaredScope;

pub struct OpenidScope;

impl DeclaredScope for OpenidScope {
    const NAME: &'static str = "openid";
}
