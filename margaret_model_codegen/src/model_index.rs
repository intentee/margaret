use crate::index_kind::IndexKind;
use crate::model_field::ModelField;
use crate::resolved_column::ResolvedColumn;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ModelIndex {
    pub fields: Vec<ModelField>,
    pub kind: IndexKind,
    pub name: String,
}

impl ModelIndex {
    #[must_use]
    pub fn column_names(&self) -> Vec<String> {
        self.columns().map(|column| column.name.clone()).collect()
    }

    pub fn columns(&self) -> impl Iterator<Item = &ResolvedColumn> {
        self.fields.iter().flat_map(|field| field.columns.iter())
    }

    #[must_use]
    pub fn field_names(&self) -> Vec<String> {
        self.fields.iter().map(|field| field.name.clone()).collect()
    }
}
