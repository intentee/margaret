#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BearerToken {
    value: String,
}

impl BearerToken {
    pub(crate) fn new(value: String) -> Self {
        Self { value }
    }

    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.value
    }
}
