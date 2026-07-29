use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::sync::Arc;

use proc_macro2::Ident;
use proc_macro2::TokenStream;
use quote::format_ident;
use quote::quote;

use margaret_attributes::canonical_path::CanonicalPath;
use margaret_codegen_tokens::console_argument_ident::console_argument_ident;
use margaret_console_argument_codegen::console_argument::ConsoleArgument;
use margaret_console_argument_codegen::owned_weave::owned_weave;
use margaret_console_argument_codegen::serve_input_key::ServeInputKey;

use crate::bootstrap_arguments_literal::bootstrap_arguments_literal;
use crate::bootstrap_arguments_module::bootstrap_arguments_module;
use crate::bootstrap_arguments_type::bootstrap_arguments_type;
use crate::console_argument_binding::ConsoleArgumentBinding;
use crate::container_error::ContainerError;
use crate::container_plan::ContainerPlan;
use crate::injected_dependency::InjectedDependency;
use crate::provider_binding::ProviderBinding;
use crate::provider_console_arguments::ProviderConsoleArguments;

fn bootstrap_arguments_path(function: &Ident) -> TokenStream {
    let module = bootstrap_arguments_module(function);
    let arguments_type = bootstrap_arguments_type(function);

    quote! { super::container::build::#module::#arguments_type }
}

pub struct ContainerBindings {
    arguments: Arc<[ConsoleArgument]>,
    asynchronous_constructions: BTreeSet<String>,
    fallible_constructions: BTreeSet<String>,
    concrete_providers: BTreeMap<CanonicalPath, ProviderConsoleArguments>,
    providers: BTreeMap<CanonicalPath, ProviderBinding>,
    slots: Arc<BTreeMap<ServeInputKey, usize>>,
}

impl ContainerBindings {
    pub(crate) fn from_plan(plan: &ContainerPlan) -> Self {
        let mut providers = BTreeMap::new();
        let mut asynchronous_constructions = BTreeSet::new();
        let mut fallible_constructions = BTreeSet::new();
        let mut concrete_providers = BTreeMap::new();

        for entry in plan.planned_entries() {
            if entry.is_async {
                asynchronous_constructions.insert(entry.provider.field_name.clone());
            }

            if entry.is_fallible {
                fallible_constructions.insert(entry.provider.field_name.clone());
            }

            if plan.injectable(&entry.key) {
                providers.insert(
                    entry.key.clone(),
                    ProviderBinding {
                        field_name: entry.provider.field_name.clone(),
                        type_name: entry.provider.type_name.clone(),
                    },
                );
            }
            concrete_providers.insert(
                entry.provider.concrete_path.clone(),
                ProviderConsoleArguments {
                    arguments: Arc::clone(&entry.console_arguments),
                    slots: Arc::clone(&entry.console_slots),
                },
            );
        }

        Self {
            arguments: plan.arguments(),
            asynchronous_constructions,
            concrete_providers,
            fallible_constructions,
            providers,
            slots: plan.slots(),
        }
    }

    #[must_use]
    pub fn accessor_invocation(&self, container: &Ident, field_name: &str) -> TokenStream {
        let accessor = format_ident!("{field_name}");

        quote! { #container.#accessor() }
    }

    /// # Errors
    ///
    /// Returns `ContainerError::MissingConsoleClosure`.
    pub fn console_arguments(
        &self,
        concrete_path: &CanonicalPath,
    ) -> Result<&ProviderConsoleArguments, ContainerError> {
        self.concrete_providers.get(concrete_path).ok_or_else(|| {
            ContainerError::MissingConsoleClosure {
                path: concrete_path.to_string(),
            }
        })
    }

    /// # Errors
    ///
    /// Returns `ContainerError::MissingConsoleSlot`.
    pub fn console_slot(&self, key: &ServeInputKey) -> Result<usize, ContainerError> {
        self.slots
            .get(key)
            .copied()
            .ok_or_else(|| ContainerError::MissingConsoleSlot {
                key: format!("{key:?}"),
            })
    }

    /// # Errors
    ///
    /// Returns `ContainerError` propagated from the work it performs.
    pub fn console_union(
        &self,
        arguments: &[ConsoleArgument],
    ) -> Result<Vec<ConsoleArgument>, ContainerError> {
        let mut seen: BTreeSet<ServeInputKey> = BTreeSet::new();
        let mut unified: Vec<(usize, ConsoleArgument)> = Vec::new();

        for argument in arguments {
            let key = argument.slot_key();

            if seen.insert(key.clone()) {
                unified.push((self.console_slot(&key)?, argument.clone()));
            }
        }

        unified.sort_by_key(|(slot, _)| *slot);

        Ok(unified.into_iter().map(|(_, argument)| argument).collect())
    }

    /// # Errors
    ///
    /// Returns `ContainerError` propagated from the work it performs.
    pub fn console_weaves_owned(
        &self,
        arguments: &[ConsoleArgument],
    ) -> Result<Vec<TokenStream>, ContainerError> {
        arguments
            .iter()
            .map(|argument| self.materialize(argument))
            .collect()
    }

    #[must_use]
    pub fn construction_invocation(
        &self,
        field_name: &str,
        arguments: &[ConsoleArgumentBinding],
    ) -> TokenStream {
        let function = format_ident!("construct_{field_name}");
        let literal = bootstrap_arguments_literal(&bootstrap_arguments_path(&function), arguments);
        let invocation = quote! { super::container::build::#function(#literal) };

        if self.asynchronous_constructions.contains(field_name) {
            quote! { #invocation.await }
        } else {
            invocation
        }
    }

    #[must_use]
    pub fn construction_is_async(&self, field_name: &str) -> bool {
        self.asynchronous_constructions.contains(field_name)
    }

    #[must_use]
    pub fn construction_is_fallible(&self, field_name: &str) -> bool {
        self.fallible_constructions.contains(field_name)
    }

    #[must_use]
    pub fn serve_is_fallible(&self) -> bool {
        !self.fallible_constructions.is_empty()
    }

    #[must_use]
    pub fn serve_invocation(&self, arguments: &[ConsoleArgumentBinding]) -> TokenStream {
        let literal = bootstrap_arguments_literal(
            &bootstrap_arguments_path(&format_ident!("serve")),
            arguments,
        );
        let invocation = quote! { super::container::build::serve(#literal) };

        if self.asynchronous_constructions.is_empty() {
            invocation
        } else {
            quote! { #invocation.await }
        }
    }

    /// # Errors
    ///
    /// Returns `ContainerError` propagated from the work it performs.
    pub fn injected_console_arguments(
        &self,
        dependency: &InjectedDependency,
    ) -> Result<Vec<ConsoleArgument>, ContainerError> {
        Ok(self.console_arguments(&dependency.concrete)?.arguments.to_vec())
    }

    #[must_use]
    pub fn provider(&self, provider_key: &CanonicalPath) -> Option<&ProviderBinding> {
        self.providers.get(provider_key)
    }

    #[must_use]
    pub fn provides(&self, provider_key: &CanonicalPath) -> bool {
        self.providers.contains_key(provider_key)
    }

    /// # Errors
    ///
    /// Returns `ContainerError` propagated from the work it performs.
    pub fn serve_arguments(
        &self,
        roots: &[CanonicalPath],
        woven: &[ConsoleArgument],
    ) -> Result<Vec<ConsoleArgument>, ContainerError> {
        let mut collected: Vec<ConsoleArgument> = Vec::new();

        for root in roots {
            collected.extend_from_slice(&self.console_arguments(root)?.arguments);
        }

        collected.extend_from_slice(woven);

        self.console_union(&collected)
    }

    #[must_use]
    pub fn all_console_arguments(&self) -> &[ConsoleArgument] {
        &self.arguments
    }

    fn materialize(&self, argument: &ConsoleArgument) -> Result<TokenStream, ContainerError> {
        let slot = self.console_slot(&argument.slot_key())?;
        let ident = console_argument_ident(slot);

        Ok(owned_weave(argument, &quote! { #ident }, false))
    }
}
