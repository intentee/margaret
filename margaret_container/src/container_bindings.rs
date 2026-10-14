use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::sync::Arc;

use proc_macro2::Ident;
use proc_macro2::TokenStream;
use quote::format_ident;
use quote::quote;

use margaret_attributes::canonical_path::CanonicalPath;
use margaret_attributes::tag::Tag;
use margaret_codegen_tokens::path_tokens::path_tokens;
use margaret_codegen_tokens::positional_naming::PositionalNaming;
use margaret_umbrella_path::umbrella_item_path::umbrella_item_path;

use crate::bootstrap_arguments_literal::bootstrap_arguments_literal;
use crate::bootstrap_arguments_module::bootstrap_arguments_module;
use crate::bootstrap_arguments_type::bootstrap_arguments_type;
use crate::constructor_outcome::ConstructorOutcome;
use crate::container_error::ContainerError;
use crate::container_plan::ContainerPlan;
use crate::framework_injection_role::FrameworkInjectionRole;
use crate::injected_dependency::InjectedDependency;
use crate::provider_binding::ProviderBinding;
use crate::serve_input_binding::ServeInputBinding;
use crate::slotted_serve_input::SlottedServeInput;
use crate::unify_serve_inputs::unify_serve_inputs;

fn umbrella_arguments_type(function: &Ident) -> TokenStream {
    path_tokens(&umbrella_item_path(&[
        "container",
        "build",
        "arguments",
        &bootstrap_arguments_module(function).to_string(),
        &bootstrap_arguments_type(function).to_string(),
    ]))
}

fn bootstrap_arguments_path(function: &Ident) -> TokenStream {
    let module = bootstrap_arguments_module(function);
    let arguments_type = bootstrap_arguments_type(function);

    quote! { super::container::build::arguments::#module::#arguments_type }
}

pub struct ContainerBindings {
    asynchronous_constructions: BTreeSet<CanonicalPath>,
    concrete_providers: BTreeMap<CanonicalPath, Arc<[SlottedServeInput]>>,
    fallible_constructions: BTreeSet<CanonicalPath>,
    providers: BTreeMap<CanonicalPath, ProviderBinding>,
    serve_input_naming: PositionalNaming,
}

impl ContainerBindings {
    pub(crate) fn from_plan(plan: &ContainerPlan) -> Self {
        let mut providers = BTreeMap::new();
        let mut asynchronous_constructions = BTreeSet::new();
        let mut concrete_providers = BTreeMap::new();
        let mut fallible_constructions = BTreeSet::new();

        for entry in plan.planned_entries() {
            if entry.is_async {
                asynchronous_constructions.insert(entry.key.clone());
            }

            if let ConstructorOutcome::Fallible = entry.outcome {
                fallible_constructions.insert(entry.key.clone());
            }

            if plan.injectable(&entry.key) {
                providers.insert(
                    entry.key.clone(),
                    ProviderBinding {
                        field_name: entry.provider.field_name.clone(),
                        injection: entry.provider.injection.clone(),
                        type_name: entry.provider.type_name.clone(),
                    },
                );
            }
            concrete_providers.insert(
                entry.provider.concrete_path.clone(),
                Arc::clone(&entry.serve_inputs),
            );
        }

        Self {
            asynchronous_constructions,
            concrete_providers,
            fallible_constructions,
            providers,
            serve_input_naming: plan.serve_input_naming(),
        }
    }

    #[must_use]
    pub fn accessor_invocation(&self, container: &Ident, field_name: &str) -> TokenStream {
        let accessor = format_ident!("{field_name}");

        quote! { #container.#accessor() }
    }

    #[must_use]
    pub fn construction_arguments(
        &self,
        field_name: &str,
        arguments: &[ServeInputBinding],
    ) -> TokenStream {
        bootstrap_arguments_literal(
            &bootstrap_arguments_path(&format_ident!("construct_{field_name}")),
            arguments,
        )
    }

    #[must_use]
    pub fn construction_invocation(
        &self,
        root: &CanonicalPath,
        field_name: &str,
        arguments: &TokenStream,
    ) -> TokenStream {
        let function = format_ident!("construct_{field_name}");
        let invocation = quote! { super::container::build::#function(#arguments) };

        if self.construction_is_async(root) {
            quote! { #invocation.await }
        } else {
            invocation
        }
    }

    #[must_use]
    pub fn construction_is_async(&self, root: &CanonicalPath) -> bool {
        self.asynchronous_constructions.contains(root)
    }

    #[must_use]
    pub fn construction_outcome(&self, root: &CanonicalPath) -> ConstructorOutcome {
        if self.fallible_constructions.contains(root) {
            ConstructorOutcome::Fallible
        } else {
            ConstructorOutcome::Infallible
        }
    }

    #[must_use]
    pub fn oauth_client(&self, client: &Tag) -> Option<InjectedDependency> {
        self.provider_in_role(|role| {
            matches!(role, FrameworkInjectionRole::OAuthClient(declared) if declared == client)
        })
    }

    #[must_use]
    pub fn provider(&self, provider_key: &CanonicalPath) -> Option<&ProviderBinding> {
        self.providers.get(provider_key)
    }

    /// # Errors
    ///
    /// Returns `ContainerError::MissingProviderServeInputs`.
    pub fn provider_serve_inputs(
        &self,
        concrete_path: &CanonicalPath,
    ) -> Result<&[SlottedServeInput], ContainerError> {
        self.concrete_providers
            .get(concrete_path)
            .map(AsRef::as_ref)
            .ok_or_else(|| ContainerError::MissingProviderServeInputs {
                path: concrete_path.to_string(),
            })
    }

    #[must_use]
    pub fn provides(&self, provider_key: &CanonicalPath) -> bool {
        self.providers.contains_key(provider_key)
    }

    #[must_use]
    pub fn serve_arguments(&self, arguments: &[ServeInputBinding]) -> TokenStream {
        bootstrap_arguments_literal(
            &bootstrap_arguments_path(&format_ident!("serve")),
            arguments,
        )
    }

    #[must_use]
    pub fn serve_arguments_type(&self) -> TokenStream {
        umbrella_arguments_type(&format_ident!("serve"))
    }

    #[must_use]
    pub fn serve_input_naming(&self) -> PositionalNaming {
        self.serve_input_naming
    }

    /// # Errors
    ///
    /// Returns `ContainerError` propagated from the work it performs.
    pub fn serve_inputs(
        &self,
        roots: &[CanonicalPath],
    ) -> Result<Vec<SlottedServeInput>, ContainerError> {
        let provided = roots
            .iter()
            .map(|root| self.provider_serve_inputs(root))
            .collect::<Result<Vec<_>, _>>()?;

        Ok(unify_serve_inputs(provided.into_iter().flatten()))
    }

    #[must_use]
    pub fn serve_invocation(
        &self,
        arguments: &TokenStream,
        served_roots: &[CanonicalPath],
    ) -> TokenStream {
        let invocation = quote! { super::container::build::serve(#arguments) };

        if served_roots
            .iter()
            .any(|root| self.construction_is_async(root))
        {
            quote! { #invocation.await }
        } else {
            invocation
        }
    }

    #[must_use]
    pub fn serve_outcome(&self, served_roots: &[CanonicalPath]) -> ConstructorOutcome {
        served_roots
            .iter()
            .fold(ConstructorOutcome::Infallible, |outcome, root| {
                outcome.combined_with(self.construction_outcome(root))
            })
    }

    #[must_use]
    pub fn trusted_issuer(&self, issuer: &Tag) -> Option<InjectedDependency> {
        self.provider_in_role(|role| {
            matches!(role, FrameworkInjectionRole::TrustedIssuer(trusted) if trusted == issuer)
        })
    }

    fn provider_in_role(
        &self,
        admits: impl Fn(&FrameworkInjectionRole) -> bool,
    ) -> Option<InjectedDependency> {
        self.providers
            .iter()
            .find(|(_, binding)| admits(&binding.injection))
            .map(|(provided, binding)| InjectedDependency {
                concrete: provided.clone(),
                field: binding.field_name.clone(),
            })
    }
}
