use std::collections::BTreeMap;
use std::collections::BTreeSet;

use proc_macro2::TokenStream;
use quote::quote;

use margaret_attributes::canonical_path::CanonicalPath;
use margaret_codegen_tokens::console_argument_argument::console_argument_argument;
use margaret_codegen_tokens::console_argument_borrow::console_argument_borrow;
use margaret_codegen_tokens::console_argument_clone::console_argument_clone;
use margaret_codegen_tokens::console_argument_deref::console_argument_deref;
use margaret_codegen_tokens::console_argument_ident::console_argument_ident;
use margaret_codegen_tokens::console_argument_parameter::console_argument_parameter;
use margaret_codegen_tokens::console_argument_to_owned::console_argument_to_owned;
use margaret_console_argument_codegen::console_argument::ConsoleArgument;
use margaret_console_argument_codegen::unify_by_key::unify_by_key;
use margaret_console_argument_codegen::weaving_kind::WeavingKind;

use crate::console_closures::ConsoleClosures;
use crate::container_error::ContainerError;
use crate::container_plan::ContainerPlan;
use crate::injected_dependency::InjectedDependency;
use crate::provided_type::ProvidedType;
use crate::provider_binding::ProviderBinding;

pub struct ContainerBindings {
    accessor_console_arguments: BTreeMap<String, Vec<ConsoleArgument>>,
    collections: BTreeMap<CanonicalPath, Vec<String>>,
    console_arguments: BTreeMap<CanonicalPath, Vec<ConsoleArgument>>,
    console_slots: BTreeMap<String, usize>,
    providers: BTreeMap<CanonicalPath, ProviderBinding>,
}

impl ContainerBindings {
    pub(crate) fn from_plan(plan: &ContainerPlan, closures: &ConsoleClosures) -> Self {
        let providers = plan
            .providers
            .iter()
            .map(|(provider_key, provider)| {
                (
                    provider_key.clone(),
                    ProviderBinding {
                        field_name: provider.field_name.clone(),
                        is_interface: matches!(provider.provided, ProvidedType::Interface(_)),
                    },
                )
            })
            .collect();
        let collections = plan
            .collections
            .entries()
            .map(|(trait_path, members)| {
                let member_fields = members
                    .iter()
                    .map(|member_key| plan.providers[member_key].field_name.clone())
                    .collect();

                (trait_path.clone(), member_fields)
            })
            .collect();
        let console_arguments = plan
            .providers
            .iter()
            .chain(plan.constructions.iter())
            .map(|(entry_key, entry)| {
                (entry.concrete_path.clone(), closures.of(entry_key).to_vec())
            })
            .collect();
        let accessor_console_arguments = plan
            .providers
            .iter()
            .chain(plan.constructions.iter())
            .map(|(entry_key, entry)| (entry.field_name.clone(), closures.of(entry_key).to_vec()))
            .collect();
        let console_slots = closures.slots().clone();

        Self {
            accessor_console_arguments,
            collections,
            console_arguments,
            console_slots,
            providers,
        }
    }

    #[must_use]
    pub fn console_arguments(&self, concrete_path: &CanonicalPath) -> &[ConsoleArgument] {
        match self.console_arguments.get(concrete_path) {
            Some(arguments) => arguments,
            None => &[],
        }
    }

    #[must_use]
    pub fn console_borrows(&self, arguments: &[ConsoleArgument]) -> Vec<TokenStream> {
        arguments
            .iter()
            .map(|argument| console_argument_borrow(self.console_slot(argument.name())))
            .collect()
    }

    #[must_use]
    pub fn console_forwards(&self, arguments: &[ConsoleArgument]) -> Vec<TokenStream> {
        arguments
            .iter()
            .map(|argument| console_argument_argument(self.console_slot(argument.name())))
            .collect()
    }

    #[must_use]
    pub fn console_parameters(&self, arguments: &[ConsoleArgument]) -> Vec<TokenStream> {
        arguments
            .iter()
            .map(|argument| {
                console_argument_parameter(
                    self.console_slot(argument.name()),
                    &argument.parameter_referent(),
                )
            })
            .collect()
    }

    #[must_use]
    pub fn console_slot(&self, name: &str) -> usize {
        self.console_slots[name]
    }

    #[must_use]
    pub fn console_union(&self, arguments: &[ConsoleArgument]) -> Vec<ConsoleArgument> {
        let mut seen: BTreeSet<String> = BTreeSet::new();
        let mut unified: Vec<ConsoleArgument> = Vec::new();

        for argument in arguments {
            if seen.insert(argument.name().to_string()) {
                unified.push(argument.clone());
            }
        }

        unified.sort_by_key(|argument| self.console_slot(argument.name()));

        unified
    }

    #[must_use]
    pub fn console_weaves(&self, arguments: &[ConsoleArgument]) -> Vec<TokenStream> {
        arguments
            .iter()
            .map(|argument| self.materialize(argument, false))
            .collect()
    }

    #[must_use]
    pub fn console_weaves_owned(&self, arguments: &[ConsoleArgument]) -> Vec<TokenStream> {
        arguments
            .iter()
            .map(|argument| self.materialize(argument, true))
            .collect()
    }

    #[must_use]
    pub fn injected_console_arguments(
        &self,
        dependency: &InjectedDependency,
    ) -> Vec<Vec<ConsoleArgument>> {
        match dependency {
            InjectedDependency::SingleConcrete { field, .. }
            | InjectedDependency::SingleInterface { field, .. } => {
                vec![self.accessor_console_arguments(field).to_vec()]
            }
            InjectedDependency::Collection { member_fields, .. } => member_fields
                .iter()
                .map(|member_field| self.accessor_console_arguments(member_field).to_vec())
                .collect(),
        }
    }

    #[must_use]
    pub fn provides(&self, provider_key: &CanonicalPath) -> bool {
        self.providers.contains_key(provider_key)
    }

    pub fn serve_arguments(
        &self,
        roots: &[CanonicalPath],
        woven: &[ConsoleArgument],
    ) -> Result<Vec<ConsoleArgument>, ContainerError> {
        let mut collected: Vec<ConsoleArgument> = Vec::new();

        for root in roots {
            collected.extend_from_slice(self.console_arguments(root));
        }

        collected.extend_from_slice(woven);

        Ok(unify_by_key(&collected)?)
    }

    pub(crate) fn collection_members(&self, trait_path: &CanonicalPath) -> &[String] {
        match self.collections.get(trait_path) {
            Some(member_fields) => member_fields,
            None => &[],
        }
    }

    pub(crate) fn provider(&self, provider_key: &CanonicalPath) -> Option<&ProviderBinding> {
        self.providers.get(provider_key)
    }

    fn accessor_console_arguments(&self, field: &str) -> &[ConsoleArgument] {
        &self.accessor_console_arguments[field]
    }

    fn materialize(&self, argument: &ConsoleArgument, owned_source: bool) -> TokenStream {
        let slot = self.console_slot(argument.name());

        match argument.weaving() {
            WeavingKind::Copy => {
                if owned_source {
                    let ident = console_argument_ident(slot);

                    quote! { #ident }
                } else {
                    console_argument_deref(slot)
                }
            }
            WeavingKind::BorrowedStr | WeavingKind::BorrowedPath => console_argument_to_owned(slot),
            WeavingKind::Cloned => console_argument_clone(slot),
        }
    }
}
