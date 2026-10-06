use serde::Deserialize;
use serde::Deserializer;

/// # Errors
///
/// Returns the error of a parameter value that is not a string.
pub fn optional_parameter<'de, TDeserializer: Deserializer<'de>>(
    deserializer: TDeserializer,
) -> Result<Option<String>, TDeserializer::Error> {
    Option::<String>::deserialize(deserializer).map(|value| value.filter(|value| !value.is_empty()))
}
