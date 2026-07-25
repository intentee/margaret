use crate::struct_shape::StructShape;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ItemKind {
    Enum,
    Function,
    Module,
    Struct(StructShape),
    Trait,
}

impl ItemKind {
    #[must_use]
    pub fn is_enum(&self) -> bool {
        matches!(self, ItemKind::Enum)
    }

    #[must_use]
    pub fn is_struct(&self) -> bool {
        matches!(self, ItemKind::Struct(_))
    }
}

#[cfg(test)]
mod tests {
    use super::ItemKind;
    use crate::struct_shape::StructShape;

    #[test]
    fn is_struct_is_true_only_for_structs() {
        assert!(ItemKind::Struct(StructShape::Unit).is_struct());
        assert!(!ItemKind::Trait.is_struct());
    }

    #[test]
    fn is_enum_is_true_only_for_enums() {
        assert!(ItemKind::Enum.is_enum());
        assert!(!ItemKind::Struct(StructShape::Unit).is_enum());
    }
}
