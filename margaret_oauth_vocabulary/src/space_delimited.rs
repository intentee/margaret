use std::collections::BTreeSet;
use std::str::FromStr;

use crate::space_delimiter::SPACE_DELIMITER;

/// # Errors
///
/// Returns the parse error of the first value that is not a `TValue`.
pub fn space_delimited<TValue: FromStr + Ord>(
    value: &str,
) -> Result<BTreeSet<TValue>, TValue::Err> {
    if value.is_empty() {
        return Ok(BTreeSet::new());
    }

    value.split(SPACE_DELIMITER).map(str::parse).collect()
}
