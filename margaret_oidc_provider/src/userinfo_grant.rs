use std::collections::BTreeSet;

use uuid::Uuid;

use margaret_oauth_vocabulary::declared_scope::DeclaredScope;
use margaret_oauth_vocabulary::scope::Scope;

#[derive(Debug, Eq, PartialEq)]
pub struct UserinfoGrant {
    pub scopes: BTreeSet<Scope>,
    pub subject: Uuid,
}

impl UserinfoGrant {
    #[must_use]
    pub fn includes<TScope: DeclaredScope>(&self) -> bool {
        self.scopes
            .iter()
            .any(|scope| scope.as_str() == TScope::NAME)
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;

    use uuid::Uuid;

    use margaret_oauth_vocabulary::declared_scope::DeclaredScope;
    use margaret_oauth_vocabulary::openid_scope::OpenidScope;
    use margaret_oauth_vocabulary::scope::Scope;

    use super::UserinfoGrant;

    struct ProfileScope;

    impl DeclaredScope for ProfileScope {
        const NAME: &'static str = "profile";
    }

    fn granted_openid() -> UserinfoGrant {
        UserinfoGrant {
            scopes: BTreeSet::from([Scope::openid()]),
            subject: Uuid::nil(),
        }
    }

    #[test]
    fn includes_a_granted_scope() {
        assert!(granted_openid().includes::<OpenidScope>());
    }

    #[test]
    fn excludes_a_scope_that_was_not_granted() {
        assert!(!granted_openid().includes::<ProfileScope>());
    }
}
