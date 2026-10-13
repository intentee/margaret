use crate::framework_attribute::FrameworkAttribute;

#[derive(Clone, Copy)]
pub(crate) enum AttributeHost {
    Item,
    Member,
}

impl AttributeHost {
    pub(crate) fn admits_repetition(self, attribute: FrameworkAttribute) -> bool {
        match self {
            Self::Item => attribute.repeats_on_items(),
            Self::Member => false,
        }
    }
}
