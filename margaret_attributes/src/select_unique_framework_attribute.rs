use crate::attribute_error::AttributeError;
use crate::framework_attribute::FrameworkAttribute;
use crate::indexed_attribute::IndexedAttribute;
use crate::select_framework_attributes::select_framework_attributes;

/// # Errors
///
/// Returns `AttributeError::RepeatedAttribute`.
pub fn select_unique_framework_attribute(
    attributes: &[IndexedAttribute],
    framework_attribute: FrameworkAttribute,
    target: impl FnOnce() -> String,
) -> Result<Option<&IndexedAttribute>, AttributeError> {
    match select_framework_attributes(attributes, framework_attribute).as_slice() {
        [] => Ok(None),
        [unique] => Ok(Some(unique)),
        _ => Err(AttributeError::RepeatedAttribute {
            attribute_path: framework_attribute.name().to_string(),
            target: target(),
        }),
    }
}
