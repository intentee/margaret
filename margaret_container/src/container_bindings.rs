use std::collections::BTreeMap;
use std::collections::BTreeSet;

use proc_macro2::Ident;
use proc_macro2::TokenStream;
use quote::format_ident;
use quote::quote;

use margaret_attributes::canonical_path::CanonicalPath;
use margaret_codegen_tokens::console_argument_clone::console_argument_clone;
use margaret_codegen_tokens::console_argument_ident::console_argument_ident;
use margaret_codegen_tokens::console_argument_to_owned::console_argument_to_owned;
use margaret_console_argument_codegen::console_argument::ConsoleArgument;
use margaret_console_argument_codegen::serve_input_key::ServeInputKey;
use margaret_console_argument_codegen::weaving_kind::WeavingKind;

use crate::console_closures::ConsoleClosures;
use crate::container_plan::ContainerPlan;
use crate::injected_dependency::InjectedDependency;
use crate::provider_binding::ProviderBinding;

pub struct ContainerBindings {
    accessor_console_arguments: BTreeMap<String, Vec<ConsoleArgument>>,
    console_arguments: BTreeMap<CanonicalPath, Vec<ConsoleArgument>>,
    console_slots: BTreeMap<ServeInputKey, usize>,
    providers: BTreeMap<CanonicalPath, ProviderBinding>,
}

impl ContainerBindings {
    pub(crate) fn from_plan(plan: &ContainerPlan, closures: &ConsoleClosures) -> Self {
        let providers = plan
            .injectable
            .iter()
            .map(|provider_key| (provider_key, plan.entry(provider_key)))
            .map(|(provider_key, provider)| {
                (
                    provider_key.clone(),
                    ProviderBinding {
                        field_name: provider.field_name.clone(),
                        type_name: provider.type_name.clone(),
                    },
                )
            })
            .collect();
        let console_arguments = plan
            .entries
            .iter()
            .map(|(entry_key, entry)| {
                (entry.concrete_path.clone(), closures.of(entry_key).to_vec())
            })
            .collect();
        let accessor_console_arguments = plan
            .entries
            .iter()
            .map(|(entry_key, entry)| (entry.field_name.clone(), closures.of(entry_key).to_vec()))
            .collect();
        let console_slots = closures.slots().clone();

        Self {
            accessor_console_arguments,
            console_arguments,
            console_slots,
            providers,
        }
    }

    #[must_use]
    pub fn accessor_invocation(&self, container: &Ident, field_name: &str) -> TokenStream {
        let accessor = format_ident!("{field_name}");

        quote! { #container.#accessor() }
    }

    #[must_use]
    pub fn console_arguments(&self, concrete_path: &CanonicalPath) -> &[ConsoleArgument] {
        match self.console_arguments.get(concrete_path) {
            Some(arguments) => arguments,
            None => &[],
        }
    }

    #[must_use]
    pub fn console_slot(&self, key: &ServeInputKey) -> usize {
        self.console_slots[key]
    }

    #[must_use]
    pub fn console_union(&self, arguments: &[ConsoleArgument]) -> Vec<ConsoleArgument> {
        let mut seen: BTreeSet<ServeInputKey> = BTreeSet::new();
        let mut unified: Vec<ConsoleArgument> = Vec::new();

        for argument in arguments {
            if seen.insert(argument.slot_key()) {
                unified.push(argument.clone());
            }
        }

        unified.sort_by_key(|argument| self.console_slot(&argument.slot_key()));

        unified
    }

    #[must_use]
    pub fn console_weaves_owned(&self, arguments: &[ConsoleArgument]) -> Vec<TokenStream> {
        arguments
            .iter()
            .map(|argument| self.materialize(argument))
            .collect()
    }

    #[must_use]
    pub fn construction_invocation(
        &self,
        field_name: &str,
        arguments: &[TokenStream],
    ) -> TokenStream {
        let function = format_ident!("construct_{field_name}");

        quote! { super::container::build::#function(#(#arguments),*).await }
    }

    #[must_use]
    pub fn injected_console_arguments(
        &self,
        dependency: &InjectedDependency,
    ) -> Vec<ConsoleArgument> {
        self.accessor_console_arguments(&dependency.field).to_vec()
    }

    #[must_use]
    pub fn provider(&self, provider_key: &CanonicalPath) -> Option<&ProviderBinding> {
        self.providers.get(provider_key)
    }

    #[must_use]
    pub fn provides(&self, provider_key: &CanonicalPath) -> bool {
        self.providers.contains_key(provider_key)
    }

    #[must_use]
    pub fn serve_arguments(
        &self,
        roots: &[CanonicalPath],
        woven: &[ConsoleArgument],
    ) -> Vec<ConsoleArgument> {
        let mut collected: Vec<ConsoleArgument> = Vec::new();

        for root in roots {
            collected.extend_from_slice(self.console_arguments(root));
        }

        collected.extend_from_slice(woven);

        self.console_union(&collected)
    }

    fn accessor_console_arguments(&self, field: &str) -> &[ConsoleArgument] {
        &self.accessor_console_arguments[field]
    }

    fn materialize(&self, argument: &ConsoleArgument) -> TokenStream {
        let slot = self.console_slot(&argument.slot_key());

        match argument.weaving() {
            WeavingKind::Copy => {
                let ident = console_argument_ident(slot);

                quote! { #ident }
            }
            WeavingKind::BorrowedStr | WeavingKind::BorrowedPath => console_argument_to_owned(slot),
            WeavingKind::Cloned => console_argument_clone(slot),
        }
    }
}
