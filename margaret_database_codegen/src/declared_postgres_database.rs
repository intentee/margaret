use margaret_attributes::attribute_index::AttributeIndex;
use margaret_attributes::framework_attribute::FrameworkAttribute;
use margaret_declaration_anchor::declaration_anchor::declaration_anchor;

use crate::database_codegen_error::DatabaseCodegenError;
use crate::postgres_database_declaration::PostgresDatabaseDeclaration;

pub enum DeclaredPostgresDatabase<'index> {
    Absent,
    Declared(PostgresDatabaseDeclaration<'index>),
}

impl<'index> DeclaredPostgresDatabase<'index> {
    /// # Errors
    ///
    /// Returns `DatabaseCodegenError` when a declaration is malformed or when more than one struct
    /// declares the database.
    pub fn read(index: &'index AttributeIndex) -> Result<Self, DatabaseCodegenError> {
        let mut declarations = Vec::new();

        for matched in index.select_framework_attribute(FrameworkAttribute::PostgresDatabase) {
            let anchor = declaration_anchor(index, &matched, FrameworkAttribute::PostgresDatabase)?;

            declarations.push(PostgresDatabaseDeclaration::read(&matched, anchor.item)?);
        }

        let mut declarations = declarations.into_iter();
        let Some(declared) = declarations.next() else {
            return Ok(Self::Absent);
        };

        match declarations.next() {
            Some(another) => Err(DatabaseCodegenError::AmbiguousPostgresDatabase {
                first: declared.anchor.canonical_path().to_string(),
                second: another.anchor.canonical_path().to_string(),
            }),
            None => Ok(Self::Declared(declared)),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::mem::discriminant;

    use margaret_attribute_arguments::attribute_arguments_error::AttributeArgumentsError;
    use margaret_attributes::attribute_error::AttributeError;
    use margaret_attributes_tests::indexed_source::IndexedSource;
    use margaret_declaration_anchor::declaration_anchor_error::DeclarationAnchorError;

    use super::DeclaredPostgresDatabase;
    use crate::database_codegen_error::DatabaseCodegenError;
    use crate::postgres_database_declaration::PostgresDatabaseDeclaration;

    fn read<TOutcome>(
        source: &str,
        outcome: impl FnOnce(Result<DeclaredPostgresDatabase, DatabaseCodegenError>) -> TOutcome,
    ) -> TOutcome {
        outcome(DeclaredPostgresDatabase::read(
            &IndexedSource::new(source).index,
        ))
    }

    #[test]
    fn reads_the_environment_variable_of_the_database_url() {
        read(
            "#[postgres_database(url_from = \"BLOG_DATABASE_URL\")]\npub struct BlogDatabase;\n",
            |read| {
                assert!(matches!(
                    read,
                    Ok(DeclaredPostgresDatabase::Declared(PostgresDatabaseDeclaration {
                        anchor,
                        url_from,
                    })) if anchor.canonical_path().to_string() == "crate::BlogDatabase"
                        && url_from.as_str() == "BLOG_DATABASE_URL"
                ));
            },
        );
    }

    #[test]
    fn finds_no_database_when_none_is_declared() {
        read("pub struct Unrelated;\n", |read| {
            assert_eq!(
                discriminant(&read.expect("the absence is read")),
                discriminant(&DeclaredPostgresDatabase::Absent)
            );
        });
    }

    #[test]
    fn rejects_a_declaration_without_a_url_source() {
        read(
            "#[postgres_database()]\npub struct BlogDatabase;\n",
            |read| {
                assert!(matches!(
                    read,
                    Err(DatabaseCodegenError::MissingUrlSource { anchor }) if anchor == "crate::BlogDatabase"
                ));
            },
        );
    }

    #[test]
    fn rejects_a_url_source_that_is_not_an_environment_variable_name() {
        read(
            "#[postgres_database(url_from = \"blog database\")]\npub struct BlogDatabase;\n",
            |read| {
                assert!(matches!(
                    read,
                    Err(DatabaseCodegenError::MalformedUrlSource { name, .. }) if name == "blog database"
                ));
            },
        );
    }

    #[test]
    fn rejects_a_url_source_that_is_not_a_string() {
        read(
            "#[postgres_database(url_from = BLOG_DATABASE_URL)]\npub struct BlogDatabase;\n",
            |read| {
                assert!(matches!(
                    read,
                    Err(DatabaseCodegenError::AttributeArguments(
                        AttributeArgumentsError::UnexpectedArgument { key, .. }
                    )) if key == "url_from"
                ));
            },
        );
    }

    #[test]
    fn rejects_an_unknown_argument() {
        read(
            "#[postgres_database(url_from = \"BLOG_DATABASE_URL\", pool = 4)]\npub struct BlogDatabase;\n",
            |read| {
                assert!(matches!(
                    read,
                    Err(DatabaseCodegenError::AttributeArguments(
                        AttributeArgumentsError::UnrecognizedArgument { argument, .. }
                    )) if argument == "pool"
                ));
            },
        );
    }

    #[test]
    fn rejects_arguments_that_do_not_parse() {
        read(
            "#[postgres_database(= 5)]\npub struct BlogDatabase;\n",
            |read| {
                assert!(matches!(
                    read,
                    Err(DatabaseCodegenError::Index(AttributeError::Arguments(
                        AttributeArgumentsError::Malformed { attribute_path, .. }
                    ))) if attribute_path == "postgres_database"
                ));
            },
        );
    }

    #[test]
    fn rejects_a_declaration_on_a_singleton() {
        read(
            "#[singleton]\n#[postgres_database(url_from = \"BLOG_DATABASE_URL\")]\npub struct BlogDatabase;\n",
            |read| {
                assert!(matches!(
                    read,
                    Err(DatabaseCodegenError::Anchor(
                        DeclarationAnchorError::DeclaredAsSingleton { path, .. }
                    )) if path == "crate::BlogDatabase"
                ));
            },
        );
    }

    #[test]
    fn rejects_two_databases() {
        read(
            "#[postgres_database(url_from = \"BLOG_DATABASE_URL\")]\npub struct BlogDatabase;\n\n#[postgres_database(url_from = \"AUDIT_DATABASE_URL\")]\npub struct AuditDatabase;\n",
            |read| {
                assert!(matches!(
                    read,
                    Err(DatabaseCodegenError::AmbiguousPostgresDatabase { first, second })
                        if first == "crate::AuditDatabase" && second == "crate::BlogDatabase"
                ));
            },
        );
    }
}
