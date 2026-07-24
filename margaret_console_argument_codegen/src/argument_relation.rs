#[derive(Clone, Debug, PartialEq)]
pub enum ArgumentRelation {
    RequiredIfEq { argument: String, value: String },
}
