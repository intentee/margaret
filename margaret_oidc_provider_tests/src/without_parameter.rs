use serde_json::Value;

/// # Panics
///
/// Panics when the parameters are not an object.
#[must_use]
pub fn without_parameter(mut parameters: Value, name: &str) -> Value {
    parameters
        .as_object_mut()
        .expect("the parameters are an object")
        .remove(name);

    parameters
}
