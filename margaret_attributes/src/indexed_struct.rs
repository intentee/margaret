use crate::identifier::Identifier;
use crate::struct_shape::StructShape;

pub struct IndexedStruct<'index> {
    pub identifier: &'index Identifier,
    pub shape: StructShape,
}
