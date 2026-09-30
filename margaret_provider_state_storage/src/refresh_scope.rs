use std::collections::BTreeSet;

use margaret_oauth_vocabulary::scope::Scope;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum RefreshScope {
    Granted,
    Narrowed(BTreeSet<Scope>),
}

impl RefreshScope {
    #[must_use]
    pub fn is_within(&self, granted: &BTreeSet<Scope>) -> bool {
        match self {
            Self::Granted => true,
            Self::Narrowed(narrowed) => narrowed.is_subset(granted),
        }
    }
}
