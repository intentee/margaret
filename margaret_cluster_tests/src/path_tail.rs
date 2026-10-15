use margaret::framework::url_path::path_tail_value::PathTailValue;
use margaret::framework::url_path::unprefixed_segment::UnprefixedSegment;

/// # Panics
///
/// Panics when the value cannot fill the tail of a path.
#[must_use]
pub fn path_tail(value: String) -> PathTailValue<UnprefixedSegment> {
    PathTailValue::new(value).expect("the value fills the tail of a path")
}
