use margaret_attributes::canonical_path::CanonicalPath;
use proc_macro2::Ident;
use proc_macro2::TokenStream;
use quote::format_ident;
use quote::quote;

use crate::container_plan::ContainerPlan;
use crate::dependency_kind::DependencyKind;
use crate::provided_type::ProvidedType;
use crate::provider::Provider;

pub(crate) fn render(plan: &ContainerPlan, order: &[usize]) -> String {
    let field_declarations = plan.providers.iter().map(field_declaration);
    let local_bindings = order
        .iter()
        .map(|&index| local_binding(&plan.providers[index], plan));
    let field_initializers = plan.providers.iter().map(field_initializer);

    let tokens = quote! {
        pub struct Container {
            #(#field_declarations,)*
        }

        impl Container {
            pub fn new() -> Self {
                #(#local_bindings)*

                Self {
                    #(#field_initializers,)*
                }
            }
        }

        impl Default for Container {
            fn default() -> Self {
                Self::new()
            }
        }
    };
    let file =
        syn::parse2::<syn::File>(tokens).expect("the generated tokens form a valid Rust file");

    prettyplease::unparse(&file)
}

fn field_declaration(provider: &Provider) -> TokenStream {
    let name = field_ident(provider);
    let field_type = field_type(provider);

    quote! { pub #name: #field_type }
}

fn field_initializer(provider: &Provider) -> TokenStream {
    let name = field_ident(provider);

    quote! { #name: #name.clone() }
}

fn field_type(provider: &Provider) -> TokenStream {
    match &provider.provided {
        ProvidedType::Concrete(path) => {
            let concrete = path_tokens(path);

            quote! { std::sync::Arc<#concrete> }
        }
        ProvidedType::Interface(path) => {
            let interface = path_tokens(path);

            quote! { std::sync::Arc<dyn #interface + Send + Sync> }
        }
    }
}

fn local_binding(provider: &Provider, plan: &ContainerPlan) -> TokenStream {
    let name = field_ident(provider);
    let concrete = path_tokens(&provider.concrete_path);
    let constructor = format_ident!("{}", provider.constructor_method);
    let arguments = provider
        .dependencies
        .iter()
        .map(|dependency| dependency_expression(dependency, plan));

    quote! {
        let #name = std::sync::Arc::new(#concrete::#constructor(#(#arguments),*));
    }
}

fn dependency_expression(dependency: &DependencyKind, plan: &ContainerPlan) -> TokenStream {
    match dependency {
        DependencyKind::Single { provider_key } => {
            let name = field_ident(provider_by_key(plan, provider_key));

            quote! { #name.clone() }
        }
        DependencyKind::Collection { trait_path } => {
            let elements = plan
                .collections
                .members_of(trait_path)
                .iter()
                .map(|member| {
                    let name = field_ident(provider_by_key(plan, member));

                    quote! { #name.clone() }
                });

            quote! { vec![#(#elements),*] }
        }
    }
}

fn provider_by_key<'plan>(plan: &'plan ContainerPlan, key: &CanonicalPath) -> &'plan Provider {
    plan.providers
        .iter()
        .find(|provider| provider.provided.key() == key)
        .expect("a resolved key maps to a provider")
}

fn field_ident(provider: &Provider) -> Ident {
    format_ident!("{}", provider.field_name)
}

fn path_tokens(path: &CanonicalPath) -> TokenStream {
    let mut segments = path.segments().iter();
    segments
        .next()
        .expect("a canonical path has at least one segment");
    let rest = segments.map(|segment| format_ident!("{}", segment));

    quote! { crate #(:: #rest)* }
}
