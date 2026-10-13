use margaret::framework::active_record::secret_text::SecretText;
use margaret::framework::macros::model;

use crate::models::credential_scope::CredentialScope;

#[model(table = "scoped_credentials")]
#[primary_key(fields = [scope, secret])]
#[derive(Clone)]
pub struct ScopedCredential {
    #[column]
    pub scope: CredentialScope,
    #[column]
    pub secret: SecretText,
    #[column]
    pub holder: String,
}
