use margaret_attributes::indexed_item::IndexedItem;
use margaret_attributes::matched_attribute::MatchedAttribute;
use margaret_environment_variable_codegen::environment_variable_name::EnvironmentVariableName;

use crate::database_codegen_error::DatabaseCodegenError;

pub struct PostgresDatabaseDeclaration<'index> {
    pub anchor: &'index IndexedItem,
    pub url_from: EnvironmentVariableName,
}

impl<'index> PostgresDatabaseDeclaration<'index> {
    pub(crate) fn read(
        matched: &MatchedAttribute,
        anchor: &'index IndexedItem,
    ) -> Result<Self, DatabaseCodegenError> {
        let path = anchor.canonical_path();

        matched.args()?.interpret(|reader| {
            let name = reader.take_string("url_from")?.ok_or_else(|| {
                DatabaseCodegenError::MissingUrlSource {
                    anchor: path.to_string(),
                }
            })?;

            EnvironmentVariableName::new(&name)
                .map(|url_from| Self { anchor, url_from })
                .ok_or_else(|| DatabaseCodegenError::MalformedUrlSource {
                    anchor: path.to_string(),
                    name,
                })
        })
    }
}
