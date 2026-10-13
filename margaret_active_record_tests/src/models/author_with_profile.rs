use margaret::framework::macros::eager_load;

use crate::models::author::Author;
use crate::models::profile::Profile;

#[eager_load(model = Author)]
#[derive(Debug, PartialEq)]
pub struct AuthorWithProfile {
    #[base]
    pub author: Author,
    #[relation(profile)]
    pub profile: Option<Profile>,
}
