use margaret_attribute_arguments::attribute_args::AttributeArgs;
use margaret_attribute_arguments::format_path::format_path;
use margaret_attributes::attribute_index::AttributeIndex;
use margaret_attributes::indexed_item::IndexedItem;
use margaret_item_naming_argument::item_naming_argument::ItemNamingArgument;
use margaret_model::on_delete::OnDelete;

use crate::model_codegen_error::ModelCodegenError;
use crate::on_delete_argument::OnDeleteArgument;

pub(crate) struct ForeignKeyArguments {
    pub(crate) on_delete: OnDelete,
}

impl ForeignKeyArguments {
    pub(crate) fn parse(
        arguments: &AttributeArgs,
        index: &AttributeIndex,
        item: &IndexedItem,
        model: &str,
        field: &str,
    ) -> Result<Self, ModelCodegenError> {
        arguments.interpret(|reader| {
            match OnDeleteArgument::of(
                index,
                item,
                reader.take_path(ItemNamingArgument::OnDelete.key())?,
            ) {
                OnDeleteArgument::Known(on_delete) => Ok(Self { on_delete }),
                OnDeleteArgument::Missing => Err(ModelCodegenError::ForeignKeyRequiresOnDelete {
                    field: field.to_string(),
                    model: model.to_string(),
                }),
                OnDeleteArgument::Unknown(path) => Err(ModelCodegenError::UnknownOnDeleteAction {
                    action: format_path(&path),
                    field: field.to_string(),
                    model: model.to_string(),
                }),
            }
        })
    }
}
