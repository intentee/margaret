use margaret::framework::macros::route_parameter_value;

#[route_parameter_value]
pub struct ProjectId(pub String);

impl From<String> for ProjectId {
    fn from(value: String) -> Self {
        Self(value)
    }
}
