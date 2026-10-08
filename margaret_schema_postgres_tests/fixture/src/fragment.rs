use uuid::Uuid;

use margaret::framework::macros::model;
use margaret::framework::model::on_delete::OnDelete;

use crate::fragment_metadata::FragmentMetadata;

#[model(table = "fragment")]
#[primary_key(columns = [partition, hash, context])]
#[unique(columns = [hash, context])]
#[foreign_key(
    columns = [partition, hash],
    references = FragmentMetadata,
    on_delete = OnDelete::Cascade
)]
pub struct Fragment {
    #[column]
    pub partition: Uuid,
    #[column]
    pub hash: Vec<u8>,
    #[column]
    pub context: Uuid,
}
