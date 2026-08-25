use margaret::framework::macros::model;

#[model(table = "fragment")]
#[foreign_key(
    columns = [partition, hash],
    references = crate::fragment_metadata::FragmentMetadata,
    on_delete = cascade
)]
pub struct FragmentAssociation {
    #[column(primary_key)]
    pub partition: uuid::Uuid,
    #[column(primary_key)]
    pub hash: Vec<u8>,
    #[column(primary_key)]
    pub context: uuid::Uuid,
}
