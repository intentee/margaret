use margaret::framework::active_record::key::Key;
use margaret::framework::macros::model;
use margaret::framework::model::on_delete::OnDelete;

use crate::fragment_metadata::FragmentMetadata;

#[model(table = "fragment")]
#[primary_key(fields = [metadata, context])]
#[index(name = "fragment_context_metadata", fields = [context, metadata])]
pub struct FragmentAssociation {
    #[column]
    #[foreign_key(on_delete = OnDelete::Cascade)]
    pub metadata: Key<FragmentMetadata>,
    #[column]
    pub context: uuid::Uuid,
}
