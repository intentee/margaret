use quote::format_ident;
use syn::Type;

use margaret_attributes::attribute_args::AttributeArgs;
use margaret_attributes::attribute_index::AttributeIndex;
use margaret_attributes::attribute_selector::AttributeSelector;
use margaret_attributes::indexed_method::IndexedMethod;
use margaret_attributes::type_leaf_ident::type_leaf_ident;
use margaret_injection_codegen::marker::marker;
use margaret_injection_codegen::parameter_view::ParameterView;
use margaret_injection_codegen::parameters::parameters;
use margaret_injection_codegen::process_method::process_method;

use crate::console_argument::ConsoleArgument;
use crate::console_argument_arguments::ConsoleArgumentArguments;
use crate::console_codegen_error::ConsoleCodegenError;
use crate::console_command::ConsoleCommand;
use crate::console_command_arguments::ConsoleCommandArguments;
use crate::optional_parameter::OptionalParameter;

fn command_arguments(
    runner: &IndexedMethod,
    argument_selector: &AttributeSelector,
    command: &str,
) -> Result<Vec<ConsoleArgument>, ConsoleCodegenError> {
    let mut arguments = Vec::new();

    for ParameterView {
        attributes,
        declared,
        position,
        ..
    } in parameters(runner.signature())
    {
        if is_cancellation_token(declared) {
            continue;
        }

        let Some(attribute) = marker(attributes, argument_selector) else {
            return Err(ConsoleCodegenError::UnmarkedProcessParameter {
                command: command.to_string(),
                parameter: position.to_string(),
            });
        };

        let attribute_arguments = AttributeArgs::from_attribute(attribute)?;
        let ConsoleArgumentArguments { from } =
            ConsoleArgumentArguments::parse(&attribute_arguments, command, position)?;

        arguments.push(console_argument(from, declared));
    }

    Ok(arguments)
}

fn console_argument(from: String, declared: &Type) -> ConsoleArgument {
    if is_bool(declared) {
        return ConsoleArgument::Flag { name: from };
    }

    let OptionalParameter {
        required,
        value_type,
    } = OptionalParameter::from_type(declared);

    if required {
        ConsoleArgument::Positional {
            id: from,
            required,
            value_type,
        }
    } else {
        ConsoleArgument::Named {
            name: from,
            required,
            value_type,
        }
    }
}

fn is_bool(declared: &Type) -> bool {
    matches!(declared, Type::Path(type_path) if type_path.path.is_ident("bool"))
}

fn is_cancellation_token(declared: &Type) -> bool {
    type_leaf_ident(declared).is_some_and(|ident| ident == "CancellationToken")
}

fn runner_takes_token(runner: &IndexedMethod) -> bool {
    parameters(runner.signature())
        .iter()
        .any(|view| is_cancellation_token(view.declared))
}

pub(crate) fn console_commands(
    index: &AttributeIndex,
) -> Result<Vec<ConsoleCommand>, ConsoleCodegenError> {
    let command_selector = AttributeSelector::parse("console_command").expect("a valid selector");
    let argument_selector = AttributeSelector::parse("console_argument").expect("a valid selector");
    let mut commands = Vec::new();

    for matched in index.select(&command_selector) {
        let item = matched.item();

        if !item.kind().is_struct() {
            return Err(ConsoleCodegenError::ConsoleCommandNotOnStruct {
                target: item.canonical_path().to_string(),
            });
        }

        let command = item.canonical_path().to_string();
        let ConsoleCommandArguments { name, description } =
            ConsoleCommandArguments::parse(matched.args()?, &command)?;
        let accessor = format_ident!("{}", index.field_name(item.canonical_path()));
        let runner = process_method(item)?;
        let arguments = command_arguments(runner, &argument_selector, &command)?;

        commands.push(ConsoleCommand {
            accessor,
            arguments,
            description,
            name,
            takes_token: runner_takes_token(runner),
        });
    }

    Ok(commands)
}
