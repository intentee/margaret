use margaret_attribute_arguments::attribute_args::AttributeArgs;
use margaret_attribute_arguments::format_path::format_path;
use margaret_attributes::attribute_index::AttributeIndex;
use margaret_attributes::canonical_path::CanonicalPath;
use margaret_attributes::framework_attribute::FrameworkAttribute;
use margaret_attributes::indexed_item::IndexedItem;
use margaret_attributes::is_snake_case_identifier::is_snake_case_identifier;
use margaret_item_naming_argument::item_naming_argument::ItemNamingArgument;

use crate::model_codegen_error::ModelCodegenError;

pub(crate) struct RelationArguments {
    pub(crate) key_field: String,
    pub(crate) name: String,
    pub(crate) related: CanonicalPath,
}

impl RelationArguments {
    pub(crate) fn parse(
        arguments: &AttributeArgs,
        declaration: FrameworkAttribute,
        index: &AttributeIndex,
        item: &IndexedItem,
        model: &str,
    ) -> Result<Self, ModelCodegenError> {
        arguments.interpret(|reader| {
            let declaration_name = declaration.name().to_string();
            let name = reader.take_string("name")?.ok_or_else(|| {
                ModelCodegenError::RelationRequiresName {
                    declaration: declaration_name.clone(),
                    model: model.to_string(),
                }
            })?;

            if !is_snake_case_identifier(&name) {
                return Err(ModelCodegenError::InvalidRelationName {
                    model: model.to_string(),
                    relation: name,
                });
            }

            let declared_model = reader
                .take_path(ItemNamingArgument::RelationModel.key())?
                .ok_or_else(|| ModelCodegenError::RelationRequiresModel {
                    model: model.to_string(),
                    relation: name.clone(),
                })?;
            let related = index
                .resolve_item_path(item, &declared_model)
                .ok_or_else(|| ModelCodegenError::RelatedItemNotAModel {
                    model: model.to_string(),
                    related: format_path(&declared_model),
                    relation: name.clone(),
                })?;
            let key =
                reader
                    .take_path("key")?
                    .ok_or_else(|| ModelCodegenError::RelationRequiresKey {
                        model: model.to_string(),
                        relation: name.clone(),
                    })?;
            let key_field = key.get_ident().map(ToString::to_string).ok_or_else(|| {
                ModelCodegenError::RelationKeyIsNotAnIdentifier {
                    key: format_path(&key),
                    model: model.to_string(),
                    relation: name.clone(),
                }
            })?;

            Ok(Self {
                key_field,
                name,
                related,
            })
        })
    }
}
