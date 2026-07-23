use syn::Type;

use margaret_attributes::attribute_index::AttributeIndex;
use margaret_attributes::canonical_path::CanonicalPath;
use margaret_attributes::indexed_item::IndexedItem;

use crate::console_argument::ConsoleArgument;
use crate::console_argument_codegen_error::ConsoleArgumentCodegenError;
use crate::console_argument_form::ConsoleArgumentForm;
use crate::optional_parameter::OptionalParameter;

fn is_bool(index: &AttributeIndex, item: &IndexedItem, declared: &Type) -> bool {
    index.resolve_item_type(item, declared) == Some(CanonicalPath::new(vec!["bool".to_string()]))
}

pub fn classify(
    index: &AttributeIndex,
    item: &IndexedItem,
    form: ConsoleArgumentForm,
    parameter: &str,
    declared: &Type,
    owner: &str,
) -> Result<ConsoleArgument, ConsoleArgumentCodegenError> {
    let bool_typed = is_bool(index, item, declared);

    match form {
        ConsoleArgumentForm::Named { key } => {
            if bool_typed {
                return Ok(ConsoleArgument::Flag { name: key });
            }

            let OptionalParameter {
                required,
                value_type,
            } = OptionalParameter::from_type(declared);

            Ok(ConsoleArgument::Named {
                name: key,
                required,
                value_type,
            })
        }
        ConsoleArgumentForm::Positional => {
            if bool_typed {
                return Err(ConsoleArgumentCodegenError::BooleanPositional {
                    owner: owner.to_string(),
                    parameter: parameter.to_string(),
                });
            }

            let OptionalParameter {
                required,
                value_type,
            } = OptionalParameter::from_type(declared);

            Ok(ConsoleArgument::Positional {
                id: parameter.to_string(),
                required,
                value_type,
            })
        }
    }
}
