use crate::attribute_error::AttributeError;
use crate::attribute_selector::AttributeSelector;
use crate::indexed_attribute::IndexedAttribute;

pub fn select_unique_attribute<'attributes>(
    attributes: &'attributes [IndexedAttribute],
    selector: &AttributeSelector,
    target: impl FnOnce() -> String,
) -> Result<Option<&'attributes IndexedAttribute>, AttributeError> {
    let mut matching = attributes
        .iter()
        .filter(|attribute| selector.matches(attribute.path()));

    let Some(unique) = matching.next() else {
        return Ok(None);
    };

    if matching.next().is_some() {
        return Err(AttributeError::RepeatedAttribute {
            attribute_path: selector.display_path(),
            target: target(),
        });
    }

    Ok(Some(unique))
}
