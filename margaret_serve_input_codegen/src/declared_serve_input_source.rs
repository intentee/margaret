use margaret_attributes::framework_attribute::FrameworkAttribute;
use margaret_attributes::indexed_attribute::IndexedAttribute;

pub(crate) struct DeclaredServeInputSource<'attributes> {
    pub(crate) attribute: &'attributes IndexedAttribute,
    pub(crate) declared_by: FrameworkAttribute,
}
