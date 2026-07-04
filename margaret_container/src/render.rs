use proc_macro2::Ident;
use proc_macro2::TokenStream;
use quote::format_ident;
use quote::quote;

use margaret_attributes::canonical_path::CanonicalPath;
use margaret_attributes::path_tokens::path_tokens;
use margaret_attributes::struct_shape::StructShape;

use crate::container_plan::ContainerPlan;
use crate::dependency_kind::DependencyKind;
use crate::direct_construction::DirectConstruction;
use crate::field_ident::field_ident;
use crate::ordered_providers::ordered_providers;
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

fn accessor(provider: &Provider, plan: &ContainerPlan) -> TokenStream {
    let name = field_ident(provider);
    let field_type = field_type(provider);
    let construction = construction(provider, plan);

    quote! {
        pub async fn #name(&self) -> #field_type {
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

fn construction(provider: &Provider, plan: &ContainerPlan) -> TokenStream {
    let value = direct_value(&provider.construction, &provider.concrete_path, plan);

    quote! { std::sync::Arc::new(#value) }
}

fn direct_value(
    direct: &DirectConstruction,
    concrete_path: &CanonicalPath,
    plan: &ContainerPlan,
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
                .map(|dependency| dependency_expression(dependency, plan));

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

fn dependency_expression(dependency: &DependencyKind, plan: &ContainerPlan) -> TokenStream {
    match dependency {
        DependencyKind::Single { provider_key } => {
            let accessor = field_ident(provider_by_key(plan, provider_key));

            access(&accessor)
        }
        DependencyKind::Collection { trait_path } => {
            let elements = plan
                .collections
                .members_of(trait_path)
                .iter()
                .map(|member| {
                    let accessor = field_ident(provider_by_key(plan, member));

                    access(&accessor)
                });

            quote! { vec![#(#elements),*] }
        }
    }
}

fn access(accessor: &Ident) -> TokenStream {
    quote! { self.#accessor().await }
}

fn provider_by_key<'plan>(plan: &'plan ContainerPlan, key: &CanonicalPath) -> &'plan Provider {
    plan.providers
        .iter()
        .find(|provider| provider.provided.key() == key)
        .expect("a resolved key maps to a provider")
}

pub(crate) fn render(plan: &ContainerPlan) -> TokenStream {
    let ordered = ordered_providers(plan);
    let fields = ordered.iter().copied().map(field_declaration);
    let accessors = ordered
        .iter()
        .copied()
        .map(|provider| accessor(provider, plan));
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
