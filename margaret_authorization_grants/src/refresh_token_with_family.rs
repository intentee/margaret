use margaret::framework::macros::eager_load;

use crate::refresh_family_record::RefreshFamilyRecord;
use crate::refresh_token_record::RefreshTokenRecord;

#[eager_load(model = RefreshTokenRecord)]
pub(crate) struct RefreshTokenWithFamily {
    #[base]
    pub(crate) token: RefreshTokenRecord,
    #[relation(family)]
    pub(crate) family: RefreshFamilyRecord,
}
