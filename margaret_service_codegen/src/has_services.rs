use margaret_attributes::attribute_index::AttributeIndex;
use margaret_attributes::attribute_selector::AttributeSelector;

pub fn has_services(index: &AttributeIndex) -> bool {
    let service = AttributeSelector::parse("service").expect("the service selector is valid");
    let ticker = AttributeSelector::parse("scheduled_with_tick_timer")
        .expect("the ticker selector is valid");

    !index.select(&service).is_empty() || !index.select(&ticker).is_empty()
}
