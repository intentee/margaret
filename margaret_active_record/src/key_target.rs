use crate::record::Record;

pub trait KeyTarget: Record {
    fn primary_key(&self) -> Self::PrimaryKey;
}
