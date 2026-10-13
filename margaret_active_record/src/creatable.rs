use crate::create::Create;
use crate::draft_record::DraftRecord;
use crate::model::Model;

pub trait Creatable: Model {
    type Draft: DraftRecord;

    #[must_use]
    fn create(draft: Self::Draft) -> Create<Self> {
        Create::new(draft)
    }
}
