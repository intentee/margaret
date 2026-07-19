use std::collections::BTreeMap;

use margaret_attributes::canonical_path::CanonicalPath;

pub(crate) struct CollectionTable {
    members: BTreeMap<CanonicalPath, Vec<CanonicalPath>>,
}

impl CollectionTable {
    pub(crate) fn new() -> Self {
        Self {
            members: BTreeMap::new(),
        }
    }

    pub(crate) fn add(&mut self, trait_path: CanonicalPath, member_key: CanonicalPath) {
        self.members.entry(trait_path).or_default().push(member_key);
    }

    pub(crate) fn members_of(&self, trait_path: &CanonicalPath) -> &[CanonicalPath] {
        match self.members.get(trait_path) {
            Some(members) => members,
            None => &[],
        }
    }
}
