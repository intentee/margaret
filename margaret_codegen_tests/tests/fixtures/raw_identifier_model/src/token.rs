use margaret::framework::macros::model;

#[model(table = "tokens")]
#[index(name = "tokens_by_label", fields = [label, r#type])]
pub struct Token {
    #[column(primary_key, name = "kind")]
    pub r#type: i64,
    #[column]
    pub label: String,
}
