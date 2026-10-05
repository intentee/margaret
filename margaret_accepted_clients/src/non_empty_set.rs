use std::collections::BTreeSet;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct NonEmptySet<TMember: Ord> {
    members: BTreeSet<TMember>,
}

impl<TMember: Ord> NonEmptySet<TMember> {
    #[must_use]
    pub fn of(first: TMember, others: impl IntoIterator<Item = TMember>) -> Self {
        let mut members = others.into_iter().collect::<BTreeSet<TMember>>();

        members.insert(first);

        Self { members }
    }

    #[must_use]
    pub fn members(&self) -> &BTreeSet<TMember> {
        &self.members
    }

    #[must_use]
    pub fn only(&self) -> Option<&TMember> {
        if self.members.len() == 1 {
            self.members.first()
        } else {
            None
        }
    }
}
