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
use crate::construction_slot_path::construction_slot_path;
use crate::container_plan::ContainerPlan;
use crate::dependency_kind::DependencyKind;
use crate::direct_construction::DirectConstruction;
use crate::field_ident::field_ident;
use crate::ordered_providers::ordered_providers;
use crate::provided_type::ProvidedType;
use crate::provider::Provider;
use crate::provides_endpoint_path::provides_endpoint_path;

fn field_declaration(provider: &Provider) -> TokenStream {
    let name = field_ident(provider);
    let slot = construction_slot_path();
    let constructed = constructed_type(provider);

    quote! { #name: #slot<#constructed> }
}

fn accessor_return_type(provider: &Provider) -> TokenStream {
    let field_type = field_type(provider);
    let error = accessor_error_path();

    quote! { ::std::result::Result<#field_type, #error> }
}

fn field_type(provider: &Provider) -> TokenStream {
    let constructed = constructed_type(provider);

    quote! { std::sync::Arc<#constructed> }
}

fn constructed_type(provider: &Provider) -> TokenStream {
    match &provider.provided {
        ProvidedType::Concrete(path) => path_tokens(path),
        ProvidedType::Endpoint(_) => {
            let interface = path_tokens(&provides_endpoint_path());

            quote! { dyn #interface }
        }
        ProvidedType::UriSelected(trait_path) => {
            let interface = path_tokens(trait_path);

            quote! { dyn #interface }
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
    let accessor_return_type = accessor_return_type(provider);
    let singleton = provider.concrete_path.to_string();

    quote! {
        pub async fn #name(&self #(, #parameters)*) -> #accessor_return_type {
            self.#name
                .construct_once(#singleton, async move {
                    let provided: #field_type = #construction;

                    Ok(provided)
                })
                .await
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
        DirectConstruction::Constructor { .. }
        | DirectConstruction::Fieldless { .. }
        | DirectConstruction::FrameworkConstructor { .. }
        | DirectConstruction::FrameworkUnit => {
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
            is_async,
            method,
        } => {
            let constructor = format_ident!("{}", method);
            let arguments = woven_dependency_expressions(dependencies, plan, closures);
            let error = construction_error_path();
            let singleton = concrete_path.to_string();
            let call = if *is_async {
                quote! { #concrete::#constructor(#(#arguments),*).await }
            } else {
                quote! { #concrete::#constructor(#(#arguments),*) }
            };

            quote! {
                #error::wrap(
                    #singleton,
                    #call,
                )?
            }
        }
        DirectConstruction::Fieldless { shape } => fieldless_literal(&concrete, *shape),
        DirectConstruction::FrameworkConstructor {
            dependencies,
            is_async,
            method,
        } => {
            let constructor = format_ident!("{}", method);
            let arguments = woven_dependency_expressions(dependencies, plan, closures);

            if *is_async {
                quote! { #concrete::#constructor(#(#arguments),*).await }
            } else {
                quote! { #concrete::#constructor(#(#arguments),*) }
            }
        }
        DirectConstruction::FrameworkUnit => quote! { #concrete },
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

fn fieldless_literal(concrete: &TokenStream, shape: StructShape) -> TokenStream {
    match shape {
        StructShape::Named { .. } => quote! { #concrete {} },
        StructShape::Unit => quote! { #concrete },
        StructShape::Unnamed { .. } => quote! { #concrete() },
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

            quote! { self.#accessor(#(#woven),*).await? }
        }
    }
}

pub(crate) fn render(plan: &ContainerPlan, closures: &ConsoleClosures) -> TokenStream {
    let ordered = ordered_providers(plan);

    let fields = ordered
        .iter()
        .map(|(_, provider)| field_declaration(provider));
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
