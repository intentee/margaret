use serde_json::Value;

pub fn with_parameter(mut parameters: Value, name: &str, value: &str) -> Value {
    parameters[name] = Value::String(value.to_string());

    parameters
}
