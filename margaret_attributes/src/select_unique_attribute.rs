use crate::attribute_error::AttributeError;
use crate::attribute_selector::AttributeSelector;
use crate::indexed_attribute::IndexedAttribute;
use crate::select_matching_attributes::select_matching_attributes;

pub fn select_unique_attribute<'attributes>(
    attributes: &'attributes [IndexedAttribute],
    selector: &AttributeSelector,
    target: impl FnOnce() -> String,
) -> Result<Option<&'attributes IndexedAttribute>, AttributeError> {
    match select_matching_attributes(attributes, selector).as_slice() {
        [] => Ok(None),
        [unique] => Ok(Some(unique)),
        _ => Err(AttributeError::RepeatedAttribute {
            attribute_path: selector.display_path(),
            target: target(),
        }),
    }
}
