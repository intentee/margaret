use uuid::Uuid;

use margaret::framework::authorization_grants::issued_code::IssuedCode;

#[derive(Clone)]
pub enum HeldCode {
    Issued(Box<IssuedCode>),
    Redeemed { family: Uuid },
}
