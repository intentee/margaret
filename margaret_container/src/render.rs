use proc_macro2::TokenStream;
use quote::format_ident;
use quote::quote;

use margaret_attributes::canonical_path::CanonicalPath;
use margaret_attributes::struct_shape::StructShape;
use margaret_codegen_tokens::console_argument_ident::console_argument_ident;
use margaret_codegen_tokens::path_tokens::path_tokens;
use margaret_console_argument_codegen::console_argument::ConsoleArgument;

use crate::accessor_error_path::accessor_error_path;
use crate::console_closures::ConsoleClosures;
use crate::console_weave_ledger::ConsoleWeaveLedger;
use crate::construction_error_path::construction_error_path;
use crate::container_plan::ContainerPlan;
use crate::dependency_kind::DependencyKind;
use crate::direct_construction::DirectConstruction;
use crate::field_ident::field_ident;
use crate::provided_type::ProvidedType;
use crate::provider::Provider;
use crate::provides_endpoint_path::provides_endpoint_path;

fn field_declaration(provider: &Provider, fallible: bool) -> TokenStream {
    let name = field_ident(provider);
    let cell_type = cell_type(provider, fallible);

    quote! { #name: tokio::sync::OnceCell<#cell_type> }
}

fn cell_type(provider: &Provider, fallible: bool) -> TokenStream {
    let field_type = field_type(provider);

    if fallible {
        let error = accessor_error_path();

        quote! { Result<#field_type, #error> }
    } else {
        field_type
    }
}

fn field_type(provider: &Provider) -> TokenStream {
    match &provider.provided {
        ProvidedType::Concrete(path) => {
            let concrete = path_tokens(path);

            quote! { std::sync::Arc<#concrete> }
        }
        ProvidedType::Endpoint(_) => {
            let interface = path_tokens(&provides_endpoint_path());

            quote! { std::sync::Arc<dyn #interface> }
        }
        ProvidedType::UriSelected(trait_path) => {
            let interface = path_tokens(trait_path);

            quote! { std::sync::Arc<dyn #interface> }
        }
    }
}

fn accessor(
    key: &CanonicalPath,
    provider: &Provider,
    plan: &ContainerPlan,
    closures: &ConsoleClosures,
) -> TokenStream {
    let name = field_ident(provider);
    let field_type = field_type(provider);
    let parameters: Vec<TokenStream> = closures
        .of(key)
        .iter()
        .map(|argument| {
            let ident = console_argument_ident(closures.slot(&argument.slot_key()));
            let value_type = argument.field_type();

            quote! { #ident: #value_type }
        })
        .collect();
    let construction = construction(provider, plan, closures);

    if plan.fallibility.of(key) {
        let cell_type = cell_type(provider, true);

        quote! {
            pub async fn #name(&self #(, #parameters)*) -> #cell_type {
                margaret::framework::container_error::construct_once::construct_once(
                    &self.#name,
                    async move {
                        let provided: #field_type = #construction;

                        Ok(provided)
                    },
                )
                .await
            }
        }
    } else {
        quote! {
            pub async fn #name(&self #(, #parameters)*) -> #field_type {
                self.#name
                    .get_or_init(|| async move {
                        let provided: #field_type = #construction;

                        provided
                    })
                    .await
                    .clone()
            }
        }
    }
}

fn construction(
    provider: &Provider,
    plan: &ContainerPlan,
    closures: &ConsoleClosures,
) -> TokenStream {
    let value = direct_value(
        &provider.construction,
        &provider.concrete_path,
        plan,
        closures,
    );

    match &provider.construction {
        DirectConstruction::FrameworkAccessor { .. } | DirectConstruction::Resolved { .. } => value,
        DirectConstruction::Constructor { .. } | DirectConstruction::Fieldless { .. } => {
            quote! { std::sync::Arc::new(#value) }
        }
    }
}

fn framework_accessor_value(
    accessor: &str,
    dependencies: &[DependencyKind],
    plan: &ContainerPlan,
    closures: &ConsoleClosures,
) -> TokenStream {
    let source_expressions = woven_dependency_expressions(dependencies, plan, closures);
    let accessor_method = format_ident!("{accessor}");

    quote! { #(#source_expressions)*.#accessor_method() }
}

fn direct_value(
    direct: &DirectConstruction,
    concrete_path: &CanonicalPath,
    plan: &ContainerPlan,
    closures: &ConsoleClosures,
) -> TokenStream {
    let concrete = path_tokens(concrete_path);

    match direct {
        DirectConstruction::Constructor {
            dependencies,
            fallible,
            is_async: constructor_is_async,
            method,
        } => {
            let constructor = format_ident!("{}", method);
            let arguments = woven_dependency_expressions(dependencies, plan, closures);
            let call = if *constructor_is_async {
                quote! { #concrete::#constructor(#(#arguments),*).await }
            } else {
                quote! { #concrete::#constructor(#(#arguments),*) }
            };

            if *fallible {
                let error = construction_error_path();
                let singleton = concrete_path.to_string();

                quote! { #error::wrap(#singleton, #call)? }
            } else {
                call
            }
        }
        DirectConstruction::Fieldless { shape } => fieldless_literal(&concrete, *shape),
        DirectConstruction::FrameworkAccessor {
            accessor,
            dependencies,
        } => framework_accessor_value(accessor, dependencies, plan, closures),
        DirectConstruction::Resolved {
            dependencies,
            resolver,
        } => {
            let resolver = path_tokens(resolver);
            let arguments = woven_dependency_expressions(dependencies, plan, closures);

            quote! { #resolver(#(#arguments),*) }
        }
    }
}

fn woven_dependency_expressions(
    dependencies: &[DependencyKind],
    plan: &ContainerPlan,
    closures: &ConsoleClosures,
) -> Vec<TokenStream> {
    let mut ledger = ConsoleWeaveLedger::new(&body_slot_uses(dependencies, closures));

    dependencies
        .iter()
        .map(|dependency| dependency_expression(dependency, plan, closures, &mut ledger))
        .collect()
}

fn fieldless_literal(concrete: &TokenStream, shape: StructShape) -> TokenStream {
    match shape {
        StructShape::Named { .. } => quote! { #concrete {} },
        StructShape::Unit => quote! { #concrete },
        StructShape::Unnamed { .. } => quote! { #concrete() },
    }
}

fn dependency_arguments<'plan>(
    dependency: &'plan DependencyKind,
    closures: &'plan ConsoleClosures,
) -> Vec<&'plan ConsoleArgument> {
    match dependency {
        DependencyKind::ConsoleArgument { argument } => vec![argument.as_ref()],
        DependencyKind::Single { provider_key } => closures.of(provider_key).iter().collect(),
    }
}

fn body_slot_uses(dependencies: &[DependencyKind], closures: &ConsoleClosures) -> Vec<usize> {
    dependencies
        .iter()
        .flat_map(|dependency| dependency_arguments(dependency, closures))
        .map(|argument| closures.slot(&argument.slot_key()))
        .collect()
}

fn woven_arguments(
    arguments: &[ConsoleArgument],
    closures: &ConsoleClosures,
    ledger: &mut ConsoleWeaveLedger,
) -> Vec<TokenStream> {
    arguments
        .iter()
        .map(|argument| ledger.weave(argument, closures.slot(&argument.slot_key())))
        .collect()
}

fn dependency_expression(
    dependency: &DependencyKind,
    plan: &ContainerPlan,
    closures: &ConsoleClosures,
    ledger: &mut ConsoleWeaveLedger,
) -> TokenStream {
    match dependency {
        DependencyKind::ConsoleArgument { argument } => {
            ledger.weave(argument, closures.slot(&argument.slot_key()))
        }
        DependencyKind::Single { provider_key } => {
            let accessor = field_ident(&plan.providers[provider_key]);
            let woven = woven_arguments(closures.of(provider_key), closures, ledger);

            if plan.fallibility.of(provider_key) {
                quote! { self.#accessor(#(#woven),*).await? }
            } else {
                quote! { self.#accessor(#(#woven),*).await }
            }
        }
    }
}

pub(crate) fn render(plan: &ContainerPlan, closures: &ConsoleClosures) -> TokenStream {
    let mut ordered: Vec<(&CanonicalPath, &Provider)> = plan
        .providers
        .iter()
        .chain(plan.constructions.iter())
        .collect();

    ordered.sort_by(|(_, first), (_, second)| first.field_name.cmp(&second.field_name));

    let fields = ordered
        .iter()
        .map(|(key, provider)| field_declaration(provider, plan.fallibility.of(key)));
    let accessors = ordered
        .iter()
        .map(|(key, provider)| accessor(key, provider, plan, closures));
    let accessor_impl = (!ordered.is_empty()).then(|| {
        quote! {
            impl Container {
                #(#accessors)*
            }
        }
    });

    quote! {
        #[rustfmt::skip]
        pub mod build;

        pub struct Container {
            #(#fields,)*
        }

        #accessor_impl
    }
}
