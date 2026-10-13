use crate::family_opening::FamilyOpening;

pub enum RedemptionDecision<Decided> {
    Consume(Decided),
    OpenFamily {
        decided: Decided,
        opening: FamilyOpening,
    },
}
