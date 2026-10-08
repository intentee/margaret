use margaret::framework::macros::eager_load;

use crate::models::user_account::UserAccount;
use crate::models::user_session::UserSession;

#[eager_load(model = UserSession)]
pub struct SessionWithUser {
    #[base]
    pub session: UserSession,
    #[relation(user)]
    pub user: UserAccount,
}
