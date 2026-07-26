use margaret_attributes::indexed_method::IndexedMethod;
use margaret_attributes::struct_shape::StructShape;

pub(crate) enum ConstructionSource<'index> {
    Constructor(&'index IndexedMethod),
    Fieldless(StructShape),
}
