use uuid::Uuid;

use margaret::framework::active_record::key::Key;
use margaret::framework::macros::model;
use margaret::framework::model::on_delete::OnDelete;

use crate::fragment_metadata::FragmentMetadata;

#[model(table = "fragment")]
#[primary_key(fields = [metadata, context])]
#[unique(fields = [context, slot])]
#[derive(Clone, Debug, PartialEq)]
pub struct Fragment {
    #[column]
    #[foreign_key(on_delete = OnDelete::Cascade)]
    pub metadata: Key<FragmentMetadata>,
    #[column]
    pub context: Uuid,
    #[column]
    pub slot: i32,
}
