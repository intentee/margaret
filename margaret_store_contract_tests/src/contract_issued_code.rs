use margaret_authorization_grants::authorization_code_lifetime::AUTHORIZATION_CODE_LIFETIME;
use margaret_authorization_grants::issued_code::IssuedCode;
use margaret_registered_claims::numeric_date::NumericDate;

use crate::contract_grant::contract_grant;
use crate::contract_moment::contract_moment;

#[must_use]
pub fn contract_issued_code() -> IssuedCode {
    IssuedCode {
        expires_at: NumericDate::from(contract_moment()).after(AUTHORIZATION_CODE_LIFETIME),
        grant: contract_grant(),
    }
}
