use margaret::framework::macros::route_parameter_value;

#[route_parameter_value]
pub struct ShelfLabel(pub String);

impl From<String> for ShelfLabel {
    fn from(value: String) -> Self {
        Self(value)
    }
}
