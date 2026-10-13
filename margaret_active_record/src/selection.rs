use crate::clause::Clause;

#[derive(Clone)]
pub enum Selection {
    Everything,
    Matching(Clause),
}

impl Selection {
    pub(crate) fn and(self, clause: Clause) -> Self {
        match self {
            Self::Everything => Self::Matching(clause),
            Self::Matching(selected) => Self::Matching(selected.and(clause)),
        }
    }
}
