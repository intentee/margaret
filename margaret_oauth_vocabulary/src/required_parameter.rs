use serde::Deserialize;
use serde::Deserializer;
use serde::de::Error;
use serde::de::Unexpected;

/// # Errors
///
/// Returns the error of a parameter value that is not a string, or that is sent without a value
/// and so counts as omitted.
pub fn required_parameter<'de, TDeserializer: Deserializer<'de>>(
    deserializer: TDeserializer,
) -> Result<String, TDeserializer::Error> {
    String::deserialize(deserializer).and_then(|value| {
        if value.is_empty() {
            Err(TDeserializer::Error::invalid_value(
                Unexpected::Str(&value),
                &"a parameter sent with a value",
            ))
        } else {
            Ok(value)
        }
    })
}
