use std::collections::HashMap;

use crate::console_argument_codegen_error::ConsoleArgumentCodegenError;
use crate::console_argument::ConsoleArgument;

pub fn ensure_unique(
    declarations: &[(String, Vec<ConsoleArgument>)],
) -> Result<(), ConsoleArgumentCodegenError> {
    let mut declared_by: HashMap<String, String> = HashMap::new();

    for (owner, arguments) in declarations {
        for argument in arguments {
            let name = argument.name().to_string();

            if let Some(existing_owner) = declared_by.insert(name.clone(), owner.clone()) {
                return Err(ConsoleArgumentCodegenError::DuplicateConsoleArgument {
                    existing_owner,
                    name,
                    owner: owner.clone(),
                });
            }
        }
    }

    Ok(())
}
