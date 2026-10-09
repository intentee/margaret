use std::sync::Arc;

use crate::encodable::Encodable;

pub enum AssignedValue {
    Excluded,
    Given(Arc<dyn Encodable>),
    Greatest,
}
