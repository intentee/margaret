use uuid::Uuid;

use crate::issued_code::IssuedCode;

#[derive(Debug, Eq, PartialEq)]
pub enum CodeRedemption {
    AlreadyRedeemed { family: Uuid },
    Redeemed(Box<IssuedCode>),
    Unknown,
}
