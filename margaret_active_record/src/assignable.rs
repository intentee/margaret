use crate::field_set::FieldSet;
use crate::model::Model;

pub trait Assignable: Model {
    type Columns<Context>: FieldSet;
}
