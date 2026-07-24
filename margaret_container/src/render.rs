use proc_macro2::TokenStream;
use quote::format_ident;
use quote::quote;

use margaret_attributes::canonical_path::CanonicalPath;
use margaret_attributes::struct_shape::StructShape;
use margaret_codegen_tokens::console_argument_ident::console_argument_ident;
use margaret_codegen_tokens::path_tokens::path_tokens;
use margaret_console_argument_codegen::console_argument::ConsoleArgument;

use crate::console_closures::ConsoleClosures;
use crate::console_weave_ledger::ConsoleWeaveLedger;
use crate::container_plan::ContainerPlan;
use crate::dependency_kind::DependencyKind;
use crate::direct_construction::DirectConstruction;
use crate::field_ident::field_ident;
use crate::provided_type::ProvidedType;
use crate::provider::Provider;

fn field_declaration(provider: &Provider) -> TokenStream {
    let name = field_ident(provider);
    let field_type = field_type(provider);

    quote! { #name: tokio::sync::OnceCell<#field_type> }
}

fn field_type(provider: &Provider) -> TokenStream {
    match &provider.provided {
        ProvidedType::Concrete(path) => {
            let concrete = path_tokens(path);

            quote! { std::sync::Arc<#concrete> }
        }
        ProvidedType::Interface(path) => {
            let interface = path_tokens(path);

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
    let parameters = closures.of(key).iter().map(|argument| {
        let ident = console_argument_ident(closures.slot(argument.name()));
        let value_type = argument.field_type();

        quote! { #ident: #value_type }
    });
    let construction = construction(provider, plan, closures);

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

    quote! { std::sync::Arc::new(#value) }
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
            is_async: constructor_is_async,
            method,
        } => {
            let constructor = format_ident!("{}", method);
            let mut ledger = ConsoleWeaveLedger::new(&body_slot_uses(dependencies, plan, closures));
            let arguments: Vec<TokenStream> = dependencies
                .iter()
                .map(|dependency| dependency_expression(dependency, plan, closures, &mut ledger))
                .collect();

            if *constructor_is_async {
                quote! { #concrete::#constructor(#(#arguments),*).await }
            } else {
                quote! { #concrete::#constructor(#(#arguments),*) }
            }
        }
        DirectConstruction::Fieldless { shape } => fieldless_literal(&concrete, *shape),
    }
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
    plan: &'plan ContainerPlan,
    closures: &'plan ConsoleClosures,
) -> Vec<&'plan ConsoleArgument> {
    match dependency {
        DependencyKind::ConsoleArgument { argument } => vec![argument.as_ref()],
        DependencyKind::Single { provider_key } => closures.of(provider_key).iter().collect(),
        DependencyKind::Collection { trait_path } => plan
            .collections
            .members_of(trait_path)
            .iter()
            .flat_map(|member_key| closures.of(member_key).iter())
            .collect(),
    }
}

fn body_slot_uses(
    dependencies: &[DependencyKind],
    plan: &ContainerPlan,
    closures: &ConsoleClosures,
) -> Vec<usize> {
    dependencies
        .iter()
        .flat_map(|dependency| dependency_arguments(dependency, plan, closures))
        .map(|argument| closures.slot(argument.name()))
        .collect()
}

fn woven_arguments(
    arguments: &[ConsoleArgument],
    closures: &ConsoleClosures,
    ledger: &mut ConsoleWeaveLedger,
) -> Vec<TokenStream> {
    arguments
        .iter()
        .map(|argument| ledger.weave(argument, closures.slot(argument.name())))
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
            ledger.weave(argument, closures.slot(argument.name()))
        }
        DependencyKind::Single { provider_key } => {
            let accessor = field_ident(&plan.providers[provider_key]);
            let woven = woven_arguments(closures.of(provider_key), closures, ledger);

            quote! { self.#accessor(#(#woven),*).await }
        }
        DependencyKind::Collection { trait_path } => {
            let elements: Vec<TokenStream> = plan
                .collections
                .members_of(trait_path)
                .iter()
                .map(|member_key| {
                    let accessor = field_ident(&plan.providers[member_key]);
                    let woven = woven_arguments(closures.of(member_key), closures, ledger);

                    quote! { self.#accessor(#(#woven),*).await }
                })
                .collect();

            quote! { vec![#(#elements),*] }
        }
    }
}

pub(crate) fn render(plan: &ContainerPlan, closures: &ConsoleClosures) -> TokenStream {
    let mut ordered: Vec<(&CanonicalPath, &Provider)> = plan.providers.iter().collect();

    ordered.sort_by(|(_, first), (_, second)| first.field_name.cmp(&second.field_name));

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
