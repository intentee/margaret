use crate::attribute_selector::AttributeSelector;
use crate::indexed_attribute::IndexedAttribute;

#[must_use]
pub fn select_matching_attributes<'attributes>(
    attributes: &'attributes [IndexedAttribute],
    selector: &AttributeSelector,
) -> Vec<&'attributes IndexedAttribute> {
    attributes
        .iter()
        .filter(|attribute| selector.matches(attribute.path()))
        .collect()
}
