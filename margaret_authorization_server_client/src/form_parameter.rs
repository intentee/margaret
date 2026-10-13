#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FormParameter {
    pub name: &'static str,
    pub value: String,
}
