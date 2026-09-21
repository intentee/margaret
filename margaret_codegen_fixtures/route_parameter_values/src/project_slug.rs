use margaret::framework::macros::route_parameter_value;

#[route_parameter_value]
pub struct ProjectSlug(pub String);

impl From<String> for ProjectSlug {
    fn from(value: String) -> Self {
        Self(value)
    }
}
