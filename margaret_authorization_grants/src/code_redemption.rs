use uuid::Uuid;

#[derive(Debug, Eq, PartialEq)]
pub enum CodeRedemption<Decided> {
    AlreadyRedeemed { family: Uuid },
    Redeemed(Decided),
    Unknown,
}
