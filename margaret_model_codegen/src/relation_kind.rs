use margaret_attributes::framework_attribute::FrameworkAttribute;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RelationKind {
    HasMany,
    HasOne,
}

impl RelationKind {
    pub(crate) fn declaration(self) -> FrameworkAttribute {
        match self {
            Self::HasMany => FrameworkAttribute::HasMany,
            Self::HasOne => FrameworkAttribute::HasOne,
        }
    }
}
