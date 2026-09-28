use margaret::framework::macros;

use crate::models::User;
use crate::prelude;

#[prelude::singleton]
pub struct ReexportedMacroSingleton {
    user: User,
}

#[macros::singleton]
pub struct ModuleImportedMacroSingleton;
