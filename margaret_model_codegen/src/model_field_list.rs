use std::collections::HashSet;

use margaret_attribute_arguments::attribute_arguments_reader::AttributeArgumentsReader;
use margaret_attribute_arguments::format_path::format_path;
use margaret_attributes::framework_attribute::FrameworkAttribute;

use crate::model_codegen_error::ModelCodegenError;

pub(crate) struct ModelFieldList {
    pub(crate) fields: Vec<String>,
}

impl ModelFieldList {
    pub(crate) fn read(
        reader: &mut AttributeArgumentsReader,
        declaration: FrameworkAttribute,
        model: &str,
    ) -> Result<Self, ModelCodegenError> {
        let declared = reader.take_path_array("fields")?.ok_or_else(|| {
            ModelCodegenError::ModelDeclarationRequiresFields {
                declaration: declaration.name().to_string(),
                model: model.to_string(),
            }
        })?;
        let mut fields: Vec<String> = Vec::with_capacity(declared.len());
        let mut seen: HashSet<String> = HashSet::new();

        for path in &declared {
            let Some(identifier) = path.get_ident() else {
                return Err(ModelCodegenError::ModelDeclarationFieldIsNotAnIdentifier {
                    declaration: declaration.name().to_string(),
                    field: format_path(path),
                    model: model.to_string(),
                });
            };
            let field = identifier.to_string();

            if !seen.insert(field.clone()) {
                return Err(ModelCodegenError::ModelDeclarationRepeatsField {
                    declaration: declaration.name().to_string(),
                    field,
                    model: model.to_string(),
                });
            }

            fields.push(field);
        }

        if fields.is_empty() {
            return Err(ModelCodegenError::ModelDeclarationRequiresFields {
                declaration: declaration.name().to_string(),
                model: model.to_string(),
            });
        }

        if fields.len() < 2 {
            return Err(ModelCodegenError::ModelDeclarationRequiresSeveralFields {
                declaration: declaration.name().to_string(),
                model: model.to_string(),
            });
        }

        Ok(Self { fields })
    }
}
