use margaret_attributes::framework_attribute::FrameworkAttribute;

use crate::model_codegen_error::ModelCodegenError;
use crate::resolved_column::ResolvedColumn;

pub(crate) fn resolve_declared_columns<'columns>(
    declared: &[String],
    columns: &'columns [ResolvedColumn],
    declaration: FrameworkAttribute,
    model: &str,
) -> Result<Vec<&'columns ResolvedColumn>, ModelCodegenError> {
    let mut resolved: Vec<&ResolvedColumn> = Vec::with_capacity(declared.len());

    for name in declared {
        let Some(column) = columns.iter().find(|column| &column.name == name) else {
            if matches!(declaration, FrameworkAttribute::PrimaryKey) {
                return Err(ModelCodegenError::ModelPrimaryKeyColumnNotDeclared {
                    column: name.clone(),
                    model: model.to_string(),
                });
            }

            return Err(ModelCodegenError::ModelDeclarationColumnNotDeclared {
                column: name.clone(),
                declaration: declaration.name().to_string(),
                model: model.to_string(),
            });
        };

        resolved.push(column);
    }

    Ok(resolved)
}
