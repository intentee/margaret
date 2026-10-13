use crate::planned_provider::PlannedProvider;

pub(crate) enum ServedContainer<'plan> {
    Served { roots: Vec<&'plan PlannedProvider> },
    Unserved,
}
