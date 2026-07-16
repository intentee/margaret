use syn::Type;

use margaret_attributes::attribute_args::AttributeArgs;
use margaret_attributes::attribute_index::AttributeIndex;
use margaret_attributes::attribute_selector::AttributeSelector;
use margaret_attributes::canonical_path::CanonicalPath;
use margaret_attributes::indexed_item::IndexedItem;
use margaret_attributes::indexed_method::IndexedMethod;
use margaret_injection_codegen::is_cancellation_token::is_cancellation_token;
use margaret_injection_codegen::marker::marker;
use margaret_injection_codegen::parameter_view::ParameterView;
use margaret_injection_codegen::parameters::parameters;

use crate::console_argument_codegen_error::ConsoleArgumentCodegenError;
use crate::console_argument::ConsoleArgument;
use crate::console_argument_arguments::ConsoleArgumentArguments;
use crate::optional_parameter::OptionalParameter;
use crate::required_argument_style::RequiredArgumentStyle;

fn classify(
    index: &AttributeIndex,
    item: &IndexedItem,
    from: String,
    declared: &Type,
    style: RequiredArgumentStyle,
) -> ConsoleArgument {
    if is_bool(index, item, declared) {
        return ConsoleArgument::Flag { name: from };
    }

    let OptionalParameter {
        required,
        value_type,
    } = OptionalParameter::from_type(declared);

    if !required {
        return ConsoleArgument::Named {
            name: from,
            required,
            value_type,
        };
    }

    match style {
        RequiredArgumentStyle::Named => ConsoleArgument::Named {
            name: from,
            required,
            value_type,
        },
        RequiredArgumentStyle::Positional => ConsoleArgument::Positional {
            id: from,
            required,
            value_type,
        },
    }
}

fn is_bool(index: &AttributeIndex, item: &IndexedItem, declared: &Type) -> bool {
    index.resolve_item_type(item, declared) == Some(CanonicalPath::new(vec!["bool".to_string()]))
}

pub fn process_arguments(
    index: &AttributeIndex,
    item: &IndexedItem,
    runner: &IndexedMethod,
    owner: &str,
    style: RequiredArgumentStyle,
) -> Result<Vec<ConsoleArgument>, ConsoleArgumentCodegenError> {
    let argument_selector = AttributeSelector::parse("console_argument").expect("a valid selector");
    let mut arguments = Vec::new();

    for ParameterView {
        attributes,
        declared,
        position,
        ..
    } in parameters(runner.signature())
    {
        if is_cancellation_token(index, item, declared) {
            continue;
        }

        let Some(attribute) = marker(attributes, &argument_selector) else {
            return Err(ConsoleArgumentCodegenError::UnmarkedProcessParameter {
                owner: owner.to_string(),
                parameter: position.to_string(),
            });
        };

        let attribute_arguments = AttributeArgs::from_attribute(attribute)?;
        let ConsoleArgumentArguments { from } =
            ConsoleArgumentArguments::parse(&attribute_arguments, owner, position)?;

        arguments.push(classify(index, item, from, declared, style));
    }

    Ok(arguments)
}
