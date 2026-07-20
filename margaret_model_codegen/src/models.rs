use std::collections::HashMap;
use std::collections::HashSet;

use margaret_attributes::attribute_index::AttributeIndex;
use margaret_attributes::attribute_selector::AttributeSelector;
use margaret_attributes::field_identifier::FieldIdentifier;
use margaret_attributes::indexed_item::IndexedItem;
use margaret_attributes::is_snake_case_identifier::is_snake_case_identifier;
use margaret_attributes::select_unique_attribute::select_unique_attribute;

use crate::column_arguments::ColumnArguments;
use crate::infer_column_type::infer_column_type;
use crate::model::Model;
use crate::model_arguments::ModelArguments;
use crate::model_codegen_error::ModelCodegenError;
use crate::resolved_column::ResolvedColumn;

fn field_display(identifier: &FieldIdentifier) -> String {
    match identifier {
        FieldIdentifier::Named(field_name) => field_name.clone(),
        FieldIdentifier::Positional(position) => position.to_string(),
    }
}

fn resolve_column_name(
    name: Option<String>,
    identifier: &FieldIdentifier,
    model: &str,
) -> Result<String, ModelCodegenError> {
    match name {
        Some(explicit) => Ok(explicit),
        None => match identifier {
            FieldIdentifier::Named(field_name) => Ok(field_name.clone()),
            FieldIdentifier::Positional(position) => {
                Err(ModelCodegenError::PositionalColumnRequiresName {
                    model: model.to_string(),
                    position: *position,
                })
            }
        },
    }
}

fn resolve_columns(
    item: &IndexedItem,
    column_selector: &AttributeSelector,
    model: &str,
) -> Result<Vec<ResolvedColumn>, ModelCodegenError> {
    let mut columns: Vec<ResolvedColumn> = Vec::new();
    let mut seen_columns: HashSet<String> = HashSet::new();

    for field in item.fields() {
        let attribute =
            select_unique_attribute(field.attributes(), column_selector, || model.to_string())?
                .ok_or_else(|| ModelCodegenError::UnattributedField {
                    field: field_display(field.identifier()),
                    model: model.to_string(),
                })?;

        let ColumnArguments { name, primary_key } = ColumnArguments::parse(attribute.args()?)?;
        let column_name = resolve_column_name(name, field.identifier(), model)?;

        if !is_snake_case_identifier(&column_name) {
            return Err(ModelCodegenError::InvalidColumnName {
                column: column_name,
                model: model.to_string(),
            });
        }

        if !seen_columns.insert(column_name.clone()) {
            return Err(ModelCodegenError::DuplicateColumnName {
                column: column_name,
                model: model.to_string(),
            });
        }

        let inferred = infer_column_type(field.ty(), model, &column_name)?;

        columns.push(ResolvedColumn {
            inferred,
            name: column_name,
            primary_key,
        });
    }

    Ok(columns)
}

pub(crate) fn models(index: &AttributeIndex) -> Result<Vec<Model>, ModelCodegenError> {
    let selector = AttributeSelector::from_marker("model");
    let column_selector = AttributeSelector::from_marker("column");
    let mut resolved: Vec<Model> = Vec::new();
    let mut seen_models: HashSet<String> = HashSet::new();
    let mut seen_tables: HashMap<String, String> = HashMap::new();

    for matched in index.select(&selector) {
        let item = matched.item();
        let model = item.canonical_path().to_string();

        if !item.kind().is_struct() {
            return Err(ModelCodegenError::ModelNotAStruct { model });
        }

        if !seen_models.insert(model.clone()) {
            return Err(ModelCodegenError::DuplicateModelDeclaration { model });
        }

        let ModelArguments { table } = ModelArguments::parse(matched.args()?, &model)?;

        if !is_snake_case_identifier(&table) {
            return Err(ModelCodegenError::InvalidTableName { model, table });
        }

        if let Some(first) = seen_tables.get(&table) {
            return Err(ModelCodegenError::DuplicateTableName {
                first: first.clone(),
                second: model,
                table,
            });
        }

        seen_tables.insert(table.clone(), model.clone());

        let columns = resolve_columns(item, &column_selector, &model)?;

        resolved.push(Model { columns, table });
    }

    resolved.sort_by(|first, second| first.table.cmp(&second.table));

    Ok(resolved)
}
