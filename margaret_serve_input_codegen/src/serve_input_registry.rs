use std::collections::BTreeMap;
use std::collections::btree_map::Entry;

use margaret_attributes::canonical_path::CanonicalPath;

use crate::registered_serve_input::RegisteredServeInput;
use crate::serve_input::ServeInput;
use crate::serve_input_codegen_error::ServeInputCodegenError;
use crate::serve_input_key::ServeInputKey;
use crate::serve_input_slots::ServeInputSlots;

pub struct ServeInputRegistry {
    entries: Vec<RegisteredServeInput>,
    slots: BTreeMap<ServeInputKey, usize>,
}

impl ServeInputRegistry {
    #[must_use]
    pub fn empty() -> Self {
        Self {
            entries: Vec::new(),
            slots: BTreeMap::new(),
        }
    }

    #[must_use]
    pub fn into_slots(self) -> ServeInputSlots {
        ServeInputSlots {
            inputs: self.entries.into_iter().map(|entry| entry.input).collect(),
            slots: self.slots,
        }
    }

    /// # Errors
    ///
    /// Returns `ServeInputCodegenError::ConflictingServeInput` or
    /// `ServeInputCodegenError::SharedPositionalConsoleArgument`.
    pub fn register(
        &mut self,
        owner: &CanonicalPath,
        input: ServeInput,
    ) -> Result<usize, ServeInputCodegenError> {
        let key = input.slot_key();
        let next_slot = self.entries.len();

        match self.slots.entry(key) {
            Entry::Occupied(entry) => {
                let slot = *entry.get();
                let registered = &self.entries[slot];

                if registered.input != input {
                    return Err(ServeInputCodegenError::ConflictingServeInput {
                        first_owner: registered.owner.to_string(),
                        name: input.name().to_string(),
                        owner: owner.to_string(),
                    });
                }

                if !input.is_shareable() {
                    return Err(ServeInputCodegenError::SharedPositionalConsoleArgument {
                        first_owner: registered.owner.to_string(),
                        name: input.name().to_string(),
                        owner: owner.to_string(),
                    });
                }

                Ok(slot)
            }
            Entry::Vacant(entry) => {
                entry.insert(next_slot);
                self.entries.push(RegisteredServeInput {
                    input,
                    owner: owner.clone(),
                });

                Ok(next_slot)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use margaret_attributes::canonical_path::CanonicalPath;
    use margaret_console_argument_codegen::console_argument::ConsoleArgument;
    use margaret_environment_variable_codegen::environment_variable::EnvironmentVariable;
    use margaret_environment_variable_codegen::environment_variable_name::EnvironmentVariableName;
    use margaret_input_weaving::input_value::InputValue;
    use margaret_input_weaving::weaving_kind::WeavingKind;

    use crate::serve_input::ServeInput;
    use crate::serve_input_codegen_error::ServeInputCodegenError;
    use crate::serve_input_key::ServeInputKey;

    use super::ServeInputRegistry;

    fn owner(name: &str) -> CanonicalPath {
        CanonicalPath::new(vec!["crate".to_string(), name.to_string()])
    }

    fn value(segments: &[&str]) -> InputValue {
        InputValue {
            required: true,
            value_type: CanonicalPath::new(
                segments
                    .iter()
                    .map(std::string::ToString::to_string)
                    .collect(),
            ),
            weaving: WeavingKind::Cloned,
        }
    }

    fn named(name: &str) -> ServeInput {
        ServeInput::ConsoleArgument(ConsoleArgument::Named {
            name: name.to_string(),
            value: value(&["std", "string", "String"]),
        })
    }

    fn positional(id: &str) -> ServeInput {
        ServeInput::ConsoleArgument(ConsoleArgument::Positional {
            id: id.to_string(),
            value: value(&["std", "string", "String"]),
        })
    }

    fn variable(name: &str) -> ServeInput {
        ServeInput::EnvironmentVariable(EnvironmentVariable {
            name: EnvironmentVariableName::new(name).expect("the name is usable"),
            value: value(&["std", "string", "String"]),
        })
    }

    fn register_all(inputs: Vec<ServeInput>) -> Result<Vec<usize>, ServeInputCodegenError> {
        let mut registry = ServeInputRegistry::empty();

        inputs
            .into_iter()
            .enumerate()
            .map(|(position, input)| registry.register(&owner(&format!("Owner{position}")), input))
            .collect()
    }

    #[test]
    fn allocates_one_slot_per_distinct_key_in_declaration_order() {
        let registry = {
            let mut registry = ServeInputRegistry::empty();

            registry
                .register(&owner("First"), named("first"))
                .expect("the first key registers");
            registry
                .register(&owner("Second"), named("second"))
                .expect("the second key registers");

            registry
        };
        let slots = registry.into_slots();

        assert_eq!(slots.inputs.len(), 2);
        assert_eq!(slots.inputs[0].name(), "first");
        assert_eq!(slots.inputs[1].name(), "second");
        assert_eq!(
            slots.slots.get(&ServeInputKey::ConsoleArgument {
                name: "second".to_string()
            }),
            Some(&1)
        );
    }

    #[test]
    fn collapses_an_identical_key_onto_one_slot() {
        assert_eq!(
            register_all(vec![named("shared"), named("shared")]).expect("the key unifies"),
            vec![0, 0]
        );
    }

    #[test]
    fn rejects_a_key_declared_with_a_different_value_type() {
        let mut registry = ServeInputRegistry::empty();

        registry
            .register(&owner("First"), named("path"))
            .expect("the first declaration registers");

        let conflicting = ServeInput::ConsoleArgument(ConsoleArgument::Named {
            name: "path".to_string(),
            value: value(&["std", "path", "PathBuf"]),
        });

        assert!(matches!(
            registry
                .register(&owner("Second"), conflicting)
                .expect_err("a differing declaration is rejected"),
            ServeInputCodegenError::ConflictingServeInput { ref first_owner, ref name, ref owner }
                if first_owner == "crate::First" && name == "path" && owner == "crate::Second"
        ));
    }

    #[test]
    fn rejects_a_positional_colliding_with_a_named_key() {
        assert!(
            register_all(vec![named("name"), positional("name")])
                .expect_err("the positional collision is rejected")
                .to_string()
                .contains("a shared serve input must be declared identically everywhere")
        );
    }

    #[test]
    fn rejects_a_named_key_colliding_with_a_positional() {
        assert!(
            register_all(vec![positional("name"), named("name")])
                .expect_err("the named collision is rejected")
                .to_string()
                .contains("a shared serve input must be declared identically everywhere")
        );
    }

    #[test]
    fn rejects_a_positional_shared_by_two_owners() {
        assert!(matches!(
            register_all(vec![positional("name"), positional("name")])
                .expect_err("a shared positional is rejected"),
            ServeInputCodegenError::SharedPositionalConsoleArgument { ref name, .. }
                if name == "name"
        ));
    }

    #[test]
    fn rejects_an_environment_variable_declared_with_a_different_value_type() {
        let mut registry = ServeInputRegistry::empty();

        registry
            .register(&owner("First"), variable("DATABASE_URL"))
            .expect("the first declaration registers");

        let conflicting = ServeInput::EnvironmentVariable(EnvironmentVariable {
            name: EnvironmentVariableName::new("DATABASE_URL").expect("the name is usable"),
            value: value(&["std", "path", "PathBuf"]),
        });

        assert!(matches!(
            registry
                .register(&owner("Second"), conflicting)
                .expect_err("a differing declaration is rejected"),
            ServeInputCodegenError::ConflictingServeInput { ref name, .. }
                if name == "DATABASE_URL"
        ));
    }

    #[test]
    fn keeps_a_console_argument_and_an_environment_variable_of_one_name_apart() {
        assert_eq!(
            register_all(vec![named("DATABASE_URL"), variable("DATABASE_URL")])
                .expect("the two namespaces do not collide"),
            vec![0, 1]
        );
    }

    #[test]
    fn keeps_a_console_argument_named_after_the_spiffe_http_client_apart_from_the_injection() {
        assert_eq!(
            register_all(vec![
                named("spiffe_http_client"),
                ServeInput::SpiffeHttpClient,
            ])
            .expect("the framework binding lives in its own namespace"),
            vec![0, 1]
        );
    }

    #[test]
    fn collapses_two_spiffe_http_client_inputs_into_one() {
        assert_eq!(
            register_all(vec![
                ServeInput::SpiffeHttpClient,
                ServeInput::SpiffeHttpClient
            ])
            .expect("two injections of the one shared client unify"),
            vec![0, 0]
        );
    }
}
