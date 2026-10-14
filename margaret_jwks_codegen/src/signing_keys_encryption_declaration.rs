use margaret_attributes::indexed_item::IndexedItem;
use margaret_attributes::matched_attribute::MatchedAttribute;
use margaret_environment_variable_codegen::environment_variable_name::EnvironmentVariableName;

use crate::jwks_codegen_error::JwksCodegenError;

pub struct SigningKeysEncryptionDeclaration<'index> {
    pub anchor: &'index IndexedItem,
    pub key_from: EnvironmentVariableName,
}

impl<'index> SigningKeysEncryptionDeclaration<'index> {
    pub(crate) fn read(
        matched: &MatchedAttribute,
        anchor: &'index IndexedItem,
    ) -> Result<Self, JwksCodegenError> {
        let path = anchor.canonical_path();

        matched.args()?.interpret(|reader| {
            let key_name = reader.take_string("key_from")?.ok_or_else(|| {
                JwksCodegenError::MissingKeySource {
                    anchor: path.to_string(),
                }
            })?;
            let key_from = EnvironmentVariableName::new(&key_name).ok_or_else(|| {
                JwksCodegenError::MalformedKeySource {
                    anchor: path.to_string(),
                    name: key_name,
                }
            })?;

            Ok(Self { anchor, key_from })
        })
    }
}
