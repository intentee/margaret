use margaret::framework::active_record::secret_text::SecretText;
use margaret::framework::macros::model;

#[model(table = "credentials")]
#[derive(Clone, Debug)]
pub struct Credential {
    #[column(primary_key)]
    pub name: String,
    #[column]
    pub secret: SecretText,
}
