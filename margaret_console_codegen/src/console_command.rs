use margaret_attributes::attribute_holder::AttributeHolder;
use margaret_attributes::attribute_index::AttributeIndex;
use margaret_attributes::attribute_selector::AttributeSelector;
use margaret_attributes::item_kind::ItemKind;
use proc_macro2::Ident;
use quote::format_ident;

use crate::console_codegen_error::ConsoleCodegenError;

pub(crate) struct ConsoleCommand {
    pub(crate) description: Option<String>,
    pub(crate) field: Ident,
    pub(crate) name: String,
}

pub(crate) fn console_commands(
    index: &AttributeIndex,
) -> Result<Vec<ConsoleCommand>, ConsoleCodegenError> {
    let selector = AttributeSelector::parse("console_command").expect("a valid selector");
    let mut commands = Vec::new();

    for matched in index.select(&selector) {
        let item = match matched.holder() {
            AttributeHolder::Item(item) if item.kind() == ItemKind::Struct => item,
            holder => {
                return Err(ConsoleCodegenError::ConsoleCommandNotOnStruct {
                    target: holder.target_path(),
                });
            }
        };

        let arguments = matched.args()?;
        let command = item.canonical_path().to_string();
        let name = arguments
            .string("name")?
            .ok_or(ConsoleCodegenError::MissingCommandName { command })?;

        commands.push(ConsoleCommand {
            description: arguments.string("description")?,
            field: format_ident!("{}", item.canonical_path().field_name()),
            name,
        });
    }

    Ok(commands)
}
