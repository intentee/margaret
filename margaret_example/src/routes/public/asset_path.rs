use margaret::framework::macros::route_parameter_value;

#[route_parameter_value]
pub struct AssetPath(pub String);

impl From<String> for AssetPath {
    fn from(value: String) -> Self {
        Self(value)
    }
}
