use uuid::Uuid;

use margaret_authorization_grants::issued_code::IssuedCode;

pub(crate) enum FixtureCode {
    Issued(Box<IssuedCode>),
    Redeemed { family: Uuid },
}
