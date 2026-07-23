use proc_macro2::Ident;
use proc_macro2::TokenStream;
use quote::format_ident;
use quote::quote;

use margaret_attributes::canonical_path::CanonicalPath;
use margaret_attributes::struct_shape::StructShape;
use margaret_codegen_tokens::path_tokens::path_tokens;
use margaret_console_argument_codegen::console_argument::ConsoleArgument;

use crate::console_closures::ConsoleClosures;
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

fn console_argument_ident(closures: &ConsoleClosures, argument: &ConsoleArgument) -> Ident {
    format_ident!("console_argument_{}", closures.slot(argument.name()))
}

fn threaded_arguments(dependency_key: &CanonicalPath, closures: &ConsoleClosures) -> Vec<Ident> {
    closures
        .of(dependency_key)
        .iter()
        .map(|argument| console_argument_ident(closures, argument))
        .collect()
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
        let ident = console_argument_ident(closures, argument);
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
            let arguments = dependencies
                .iter()
                .map(|dependency| dependency_expression(dependency, plan, closures));

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

fn dependency_expression(
    dependency: &DependencyKind,
    plan: &ContainerPlan,
    closures: &ConsoleClosures,
) -> TokenStream {
    match dependency {
        DependencyKind::ConsoleArgument { argument } => {
            let ident = console_argument_ident(closures, argument);

            quote! { #ident }
        }
        DependencyKind::Single { provider_key } => {
            let accessor = field_ident(&plan.providers[provider_key]);
            let threaded = threaded_arguments(provider_key, closures);

            quote! { self.#accessor(#(#threaded),*).await }
        }
        DependencyKind::Collection { trait_path } => {
            let elements = plan
                .collections
                .members_of(trait_path)
                .iter()
                .map(|member_key| {
                    let accessor = field_ident(&plan.providers[member_key]);
                    let threaded = threaded_arguments(member_key, closures);

                    quote! { self.#accessor(#(#threaded),*).await }
                });

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
