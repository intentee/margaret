use crate::grant_operation::GrantOperation;

#[derive(Clone, Copy)]
pub enum GrantInterference {
    Failing(GrantOperation),
    ReplayBeforeFamilyOpens,
    RevocationBeforeRefreshRotates,
    RotationBeforeRefreshRotates,
    Undisturbed,
}
