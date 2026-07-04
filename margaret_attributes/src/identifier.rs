pub struct Identifier {
    field: String,
    type_name: String,
}

impl Identifier {
    pub(crate) fn new(field: String, type_name: String) -> Self {
        Self { field, type_name }
    }

    pub fn field(&self) -> &str {
        &self.field
    }

    pub fn type_name(&self) -> &str {
        &self.type_name
    }
}
