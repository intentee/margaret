use margaret_attributes::indexed_method::IndexedMethod;
use margaret_attributes::struct_shape::StructShape;

use crate::constructor_outcome::ConstructorOutcome;

pub(crate) enum ConstructionSource<'index> {
    Constructor {
        method: &'index IndexedMethod,
        outcome: ConstructorOutcome,
    },
    Fieldless(StructShape),
}
