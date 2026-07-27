use crate::framework_attribute::FrameworkAttribute;
use crate::indexed_attribute::IndexedAttribute;

#[must_use]
pub fn select_framework_attributes(
    attributes: &[IndexedAttribute],
    framework_attribute: FrameworkAttribute,
) -> Vec<&IndexedAttribute> {
    attributes
        .iter()
        .filter(|attribute| attribute.framework_attribute() == Some(framework_attribute))
        .collect()
}
