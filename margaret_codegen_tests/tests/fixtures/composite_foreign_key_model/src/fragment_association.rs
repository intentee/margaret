use margaret::framework::macros::model;

#[model(table = "fragment")]
#[primary_key(columns = [partition, hash, context])]
#[unique(columns = [hash, context])]
#[index(name = "fragment_context_partition", columns = [context, partition])]
#[foreign_key(
    columns = [partition, hash],
    references = crate::fragment_metadata::FragmentMetadata,
    on_delete = cascade
)]
pub struct FragmentAssociation {
    #[column]
    pub partition: uuid::Uuid,
    #[column]
    pub hash: Vec<u8>,
    #[column]
    pub context: uuid::Uuid,
}
