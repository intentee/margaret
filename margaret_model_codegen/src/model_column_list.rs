use std::collections::HashSet;

use margaret_attribute_arguments::attribute_arguments_reader::AttributeArgumentsReader;
use margaret_attribute_arguments::format_path::format_path;
use margaret_attributes::framework_attribute::FrameworkAttribute;

use crate::column_list_arity::ColumnListArity;
use crate::model_codegen_error::ModelCodegenError;

pub(crate) struct ModelColumnList {
    pub(crate) columns: Vec<String>,
}

impl ModelColumnList {
    pub(crate) fn read(
        reader: &mut AttributeArgumentsReader,
        declaration: FrameworkAttribute,
        arity: ColumnListArity,
        model: &str,
    ) -> Result<Self, ModelCodegenError> {
        let declared = reader.take_path_array("columns")?.unwrap_or_default();
        let mut columns: Vec<String> = Vec::with_capacity(declared.len());
        let mut seen: HashSet<String> = HashSet::new();

        for path in &declared {
            let Some(identifier) = path.get_ident() else {
                return Err(ModelCodegenError::ModelDeclarationColumnIsNotAnIdentifier {
                    column: format_path(path),
                    declaration: declaration.name().to_string(),
                    model: model.to_string(),
                });
            };
            let column = identifier.to_string();

            if !seen.insert(column.clone()) {
                return Err(ModelCodegenError::ModelDeclarationRepeatsColumn {
                    column,
                    declaration: declaration.name().to_string(),
                    model: model.to_string(),
                });
            }

            columns.push(column);
        }

        if columns.is_empty() {
            return Err(ModelCodegenError::ModelDeclarationRequiresColumns {
                declaration: declaration.name().to_string(),
                model: model.to_string(),
            });
        }

        if matches!(arity, ColumnListArity::TwoOrMore) && columns.len() < 2 {
            return Err(ModelCodegenError::ModelDeclarationRequiresSeveralColumns {
                declaration: declaration.name().to_string(),
                model: model.to_string(),
            });
        }

        Ok(Self { columns })
    }
}

#[cfg(test)]
mod tests {
    use syn::Attribute;
    use syn::parse_quote;

    use margaret_attribute_arguments::attribute_args::AttributeArgs;
    use margaret_attributes::framework_attribute::FrameworkAttribute;

    use crate::column_list_arity::ColumnListArity;
    use crate::model_codegen_error::ModelCodegenError;
    use crate::model_column_list::ModelColumnList;

    fn read(
        attribute: &Attribute,
        declaration: FrameworkAttribute,
        arity: ColumnListArity,
    ) -> Result<Vec<String>, ModelCodegenError> {
        let arguments = AttributeArgs::from_attribute(attribute).expect("the arguments parse");

        arguments.interpret(|reader| {
            ModelColumnList::read(reader, declaration, arity, "crate::Model")
                .map(|list| list.columns)
        })
    }

    fn read_unique(attribute: &Attribute) -> Result<Vec<String>, ModelCodegenError> {
        read(
            attribute,
            FrameworkAttribute::Unique,
            ColumnListArity::TwoOrMore,
        )
    }

    #[test]
    fn reads_the_declared_columns_in_order() {
        assert_eq!(
            read_unique(&parse_quote!(#[unique(columns = [partition, hash])]))
                .expect("the column list reads"),
            vec!["partition".to_string(), "hash".to_string()]
        );
    }

    #[test]
    fn reads_a_single_column_when_one_is_enough() {
        assert_eq!(
            read(
                &parse_quote!(#[foreign_key(columns = [hash])]),
                FrameworkAttribute::ForeignKey,
                ColumnListArity::OneOrMore
            )
            .expect("the column list reads"),
            vec!["hash".to_string()]
        );
    }

    #[test]
    fn rejects_absent_columns() {
        assert!(matches!(
            read_unique(&parse_quote!(#[unique])).expect_err("an absent column list is rejected"),
            ModelCodegenError::ModelDeclarationRequiresColumns { ref declaration, .. }
                if declaration == "unique"
        ));
    }

    #[test]
    fn rejects_an_empty_column_list() {
        assert!(matches!(
            read_unique(&parse_quote!(#[unique(columns = [])]))
                .expect_err("an empty column list is rejected"),
            ModelCodegenError::ModelDeclarationRequiresColumns { ref declaration, .. }
                if declaration == "unique"
        ));
    }

    #[test]
    fn rejects_a_column_that_is_not_a_plain_identifier() {
        assert!(matches!(
            read_unique(&parse_quote!(#[unique(columns = [outer::inner, hash])]))
                .expect_err("a multi segment column path is rejected"),
            ModelCodegenError::ModelDeclarationColumnIsNotAnIdentifier { ref column, .. }
                if column == "outer::inner"
        ));
    }

    #[test]
    fn rejects_a_repeated_column() {
        assert!(matches!(
            read_unique(&parse_quote!(#[unique(columns = [hash, hash])]))
                .expect_err("a repeated column is rejected"),
            ModelCodegenError::ModelDeclarationRepeatsColumn { ref column, .. }
                if column == "hash"
        ));
    }

    #[test]
    fn rejects_a_single_column_where_several_are_required() {
        assert!(matches!(
            read_unique(&parse_quote!(#[unique(columns = [hash])]))
                .expect_err("a single column declaration is rejected"),
            ModelCodegenError::ModelDeclarationRequiresSeveralColumns { ref declaration, .. }
                if declaration == "unique"
        ));
    }
}
