use margaret_attributes::indexed_item::IndexedItem;
use margaret_attributes::matched_attribute::MatchedAttribute;
use margaret_environment_variable_codegen::environment_variable_name::EnvironmentVariableName;

use crate::database_codegen_error::DatabaseCodegenError;

pub struct PostgresDatabaseDeclaration<'index> {
    pub anchor: &'index IndexedItem,
    pub max_connections_from: EnvironmentVariableName,
    pub url_from: EnvironmentVariableName,
}

impl<'index> PostgresDatabaseDeclaration<'index> {
    pub(crate) fn read(
        matched: &MatchedAttribute,
        anchor: &'index IndexedItem,
    ) -> Result<Self, DatabaseCodegenError> {
        let path = anchor.canonical_path();

        matched.args()?.interpret(|reader| {
            let url_name = reader.take_string("url_from")?.ok_or_else(|| {
                DatabaseCodegenError::MissingUrlSource {
                    anchor: path.to_string(),
                }
            })?;
            let url_from = EnvironmentVariableName::new(&url_name).ok_or_else(|| {
                DatabaseCodegenError::MalformedUrlSource {
                    anchor: path.to_string(),
                    name: url_name,
                }
            })?;
            let max_connections_name =
                reader.take_string("max_connections_from")?.ok_or_else(|| {
                    DatabaseCodegenError::MissingMaxConnectionsSource {
                        anchor: path.to_string(),
                    }
                })?;
            let max_connections_from = EnvironmentVariableName::new(&max_connections_name)
                .ok_or_else(|| DatabaseCodegenError::MalformedMaxConnectionsSource {
                    anchor: path.to_string(),
                    name: max_connections_name,
                })?;

            if max_connections_from == url_from {
                return Err(DatabaseCodegenError::SharedEnvironmentVariable {
                    anchor: path.to_string(),
                    name: url_from.as_str().to_string(),
                });
            }

            Ok(Self {
                anchor,
                max_connections_from,
                url_from,
            })
        })
    }
}
