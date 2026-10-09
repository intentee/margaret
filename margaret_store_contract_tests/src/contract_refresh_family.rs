use std::collections::BTreeSet;

use uuid::Uuid;

use margaret_authorization_grants::refresh_family::RefreshFamily;
use margaret_authorization_grants::refresh_family_lifetime::REFRESH_FAMILY_LIFETIME;
use margaret_oauth_vocabulary::scope::Scope;
use margaret_registered_claims::numeric_date::NumericDate;

use crate::contract_moment::contract_moment;

#[must_use]
pub fn contract_refresh_family() -> RefreshFamily {
    let auth_time = contract_moment();

    RefreshFamily {
        auth_time,
        client_id: "contract-client".to_string(),
        expires_at: NumericDate::from(auth_time).after(REFRESH_FAMILY_LIFETIME),
        scopes: BTreeSet::from([Scope::openid()]),
        subject: Uuid::new_v4(),
    }
}
