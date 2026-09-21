use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::sync::Arc;

use proc_macro2::Ident;
use proc_macro2::TokenStream;
use quote::format_ident;
use quote::quote;

use margaret_attributes::canonical_path::CanonicalPath;
use margaret_codegen_tokens::serve_input_ident::serve_input_ident;
use margaret_input_weaving::owned_weave::owned_weave;
use margaret_serve_input_codegen::serve_input::ServeInput;
use margaret_serve_input_codegen::serve_input_key::ServeInputKey;

use crate::bootstrap_arguments_literal::bootstrap_arguments_literal;
use crate::bootstrap_arguments_module::bootstrap_arguments_module;
use crate::bootstrap_arguments_type::bootstrap_arguments_type;
use crate::container_error::ContainerError;
use crate::container_plan::ContainerPlan;
use crate::provider_binding::ProviderBinding;
use crate::serve_input_binding::ServeInputBinding;

fn bootstrap_arguments_path(function: &Ident) -> TokenStream {
    let module = bootstrap_arguments_module(function);
    let arguments_type = bootstrap_arguments_type(function);

    quote! { super::container::build::#module::#arguments_type }
}

struct SlottedServeInput {
    input: ServeInput,
    slot: usize,
}

pub struct ContainerBindings {
    asynchronous_constructions: BTreeSet<String>,
    injectable: BTreeSet<CanonicalPath>,
    inputs: Arc<[ServeInput]>,
    providers: BTreeMap<CanonicalPath, ProviderBinding>,
    slots: Arc<BTreeMap<ServeInputKey, usize>>,
}

impl ContainerBindings {
    pub(crate) fn from_plan(plan: &ContainerPlan) -> Self {
        let mut providers = BTreeMap::new();
        let mut asynchronous_constructions = BTreeSet::new();
        let mut injectable = BTreeSet::new();

        for entry in plan.planned_entries() {
            if entry.is_async {
                asynchronous_constructions.insert(entry.provider.field_name.clone());
            }

            if plan.injectable(&entry.key) {
                injectable.insert(entry.key.clone());
            }

            providers.insert(
                entry.provider.concrete_path.clone(),
                ProviderBinding {
                    field_name: entry.provider.field_name.clone(),
                    inputs: Arc::clone(&entry.serve_inputs),
                    slots: Arc::clone(&entry.serve_input_slots),
                    type_name: entry.provider.type_name.clone(),
                },
            );
        }

        Self {
            asynchronous_constructions,
            injectable,
            inputs: plan.inputs(),
            providers,
            slots: plan.slots(),
        }
    }

    #[must_use]
    pub fn accessor_invocation(&self, container: &Ident, field_name: &str) -> TokenStream {
        let accessor = format_ident!("{field_name}");

        quote! { #container.#accessor() }
    }

    #[must_use]
    pub fn all_serve_inputs(&self) -> &[ServeInput] {
        &self.inputs
    }

    /// # Errors
    ///
    /// Returns `ContainerError::MissingProviderBinding`.
    pub fn provider_binding(
        &self,
        concrete_path: &CanonicalPath,
    ) -> Result<&ProviderBinding, ContainerError> {
        self.providers
            .get(concrete_path)
            .ok_or_else(|| ContainerError::MissingProviderBinding {
                path: concrete_path.to_string(),
            })
    }

    /// # Errors
    ///
    /// Returns `ContainerError::MissingServeInputSlot`.
    pub fn serve_input_slot(&self, key: &ServeInputKey) -> Result<usize, ContainerError> {
        self.slots
            .get(key)
            .copied()
            .ok_or_else(|| ContainerError::MissingServeInputSlot {
                key: format!("{key:?}"),
            })
    }

    /// # Errors
    ///
    /// Returns `ContainerError` propagated from the work it performs.
    pub fn serve_input_union(
        &self,
        inputs: &[ServeInput],
    ) -> Result<Vec<ServeInput>, ContainerError> {
        let mut seen: BTreeSet<ServeInputKey> = BTreeSet::new();
        let mut unified: Vec<SlottedServeInput> = Vec::new();

        for input in inputs {
            let key = input.slot_key();

            if seen.insert(key.clone()) {
                unified.push(SlottedServeInput {
                    input: input.clone(),
                    slot: self.serve_input_slot(&key)?,
                });
            }
        }

        unified.sort_by_key(|slotted| slotted.slot);

        Ok(unified.into_iter().map(|slotted| slotted.input).collect())
    }

    /// # Errors
    ///
    /// Returns `ContainerError` propagated from the work it performs.
    pub fn serve_input_weaves_owned(
        &self,
        inputs: &[ServeInput],
    ) -> Result<Vec<TokenStream>, ContainerError> {
        inputs.iter().map(|input| self.materialize(input)).collect()
    }

    #[must_use]
    pub fn construction_invocation(
        &self,
        field_name: &str,
        arguments: &[ServeInputBinding],
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
    pub fn provider(&self, provider_key: &CanonicalPath) -> Option<&ProviderBinding> {
        if self.injectable.contains(provider_key) {
            self.providers.get(provider_key)
        } else {
            None
        }
    }

    #[must_use]
    pub fn provides(&self, provider_key: &CanonicalPath) -> bool {
        self.injectable.contains(provider_key)
    }

    /// # Errors
    ///
    /// Returns `ContainerError` propagated from the work it performs.
    pub fn serve_inputs(
        &self,
        roots: &[CanonicalPath],
        woven: &[ServeInput],
    ) -> Result<Vec<ServeInput>, ContainerError> {
        let mut collected: Vec<ServeInput> = Vec::new();

        for root in roots {
            collected.extend_from_slice(&self.provider_binding(root)?.inputs);
        }

        collected.extend_from_slice(woven);

        self.serve_input_union(&collected)
    }

    #[must_use]
    pub fn serve_invocation(&self, arguments: &[ServeInputBinding]) -> TokenStream {
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

    fn materialize(&self, input: &ServeInput) -> Result<TokenStream, ContainerError> {
        let slot = self.serve_input_slot(&input.slot_key())?;
        let ident = serve_input_ident(slot);

        Ok(owned_weave(&input.weaving(), &quote! { #ident }, false))
    }
}
