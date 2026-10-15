use margaret::framework::url_path::path_parameter_value::PathParameterValue;
use margaret::framework::url_path::unprefixed_segment::UnprefixedSegment;

/// # Panics
///
/// Panics when the value cannot fill a path segment.
#[must_use]
pub fn path_parameter(value: String) -> PathParameterValue<UnprefixedSegment> {
    PathParameterValue::new(value).expect("the value fills a path segment")
}
