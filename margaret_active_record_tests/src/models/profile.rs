use margaret::framework::active_record::key::Key;
use margaret::framework::macros::model;
use margaret::framework::model::on_delete::OnDelete;

use crate::models::author::Author;

#[model(table = "profiles")]
#[derive(Clone, Debug, PartialEq)]
pub struct Profile {
    #[column(primary_key)]
    #[foreign_key(on_delete = OnDelete::Cascade)]
    pub author: Key<Author>,
    #[column]
    pub website: String,
}
