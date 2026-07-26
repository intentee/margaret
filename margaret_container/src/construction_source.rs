use margaret_attributes::indexed_method::IndexedMethod;
use margaret_attributes::struct_shape::StructShape;

use crate::constructor_return::ConstructorReturn;

pub(crate) enum ConstructionSource<'index> {
    Constructor {
        method: &'index IndexedMethod,
        returns: ConstructorReturn,
    },
    Fieldless(StructShape),
}
