use proc_macro2::Ident;
use proc_macro2::TokenStream;
use quote::format_ident;
use quote::quote;

use margaret_attributes::canonical_path::CanonicalPath;
use margaret_attributes::struct_shape::StructShape;

use crate::container_plan::ContainerPlan;
use crate::dependency_kind::DependencyKind;
use crate::provided_type::ProvidedType;
use crate::provider::Provider;
use crate::provider_construction::ProviderConstruction;

pub(crate) fn render(plan: &ContainerPlan) -> String {
    let fields = plan.providers.iter().map(field_declaration);
    let initializers = plan.providers.iter().map(field_initializer);
    let accessors = plan
        .providers
        .iter()
        .map(|provider| accessor(provider, plan));

    let tokens = quote! {
        pub struct Container {
            #(#fields,)*
        }

        impl Container {
            pub fn build() -> Self {
                Self {
                    #(#initializers,)*
                }
            }

            #(#accessors)*
        }
    };
    let file =
        syn::parse2::<syn::File>(tokens).expect("the generated tokens form a valid Rust file");

    prettyplease::unparse(&file)
}

fn field_declaration(provider: &Provider) -> TokenStream {
    let name = field_ident(provider);
    let field_type = field_type(provider);

    quote! { #name: std::sync::OnceLock<#field_type> }
}

fn field_initializer(provider: &Provider) -> TokenStream {
    let name = field_ident(provider);

    quote! { #name: std::sync::OnceLock::new() }
}

fn field_type(provider: &Provider) -> TokenStream {
    match &provider.provided {
        ProvidedType::Concrete(path) => {
            let concrete = crate_path_tokens(path);

            quote! { std::sync::Arc<#concrete> }
        }
        ProvidedType::Interface(path) => {
            let interface = crate_path_tokens(path);

            quote! { std::sync::Arc<dyn #interface> }
        }
    }
}

fn accessor(provider: &Provider, plan: &ContainerPlan) -> TokenStream {
    let name = field_ident(provider);
    let field_type = field_type(provider);
    let construction = construction(provider, plan);

    let init = match &provider.provided {
        ProvidedType::Concrete(_) => quote! { || #construction },
        ProvidedType::Interface(_) => quote! { || -> #field_type { #construction } },
    };

    quote! {
        pub fn #name(&self) -> #field_type {
            self.#name.get_or_init(#init).clone()
        }
    }
}

fn construction(provider: &Provider, plan: &ContainerPlan) -> TokenStream {
    let concrete = crate_path_tokens(&provider.concrete_path);

    match &provider.construction {
        ProviderConstruction::Constructor {
            method,
            dependencies,
        } => {
            let constructor = format_ident!("{}", method);
            let arguments = dependencies
                .iter()
                .map(|dependency| dependency_expression(dependency, plan));

            quote! { std::sync::Arc::new(#concrete::#constructor(#(#arguments),*)) }
        }
        ProviderConstruction::Fieldless { shape } => {
            let literal = fieldless_literal(&concrete, *shape);

            quote! { std::sync::Arc::new(#literal) }
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

fn dependency_expression(dependency: &DependencyKind, plan: &ContainerPlan) -> TokenStream {
    match dependency {
        DependencyKind::Single { provider_key } => {
            let accessor = field_ident(provider_by_key(plan, provider_key));

            quote! { self.#accessor() }
        }
        DependencyKind::Collection { trait_path } => {
            let elements = plan
                .collections
                .members_of(trait_path)
                .iter()
                .map(|member| {
                    let accessor = field_ident(provider_by_key(plan, member));

                    quote! { self.#accessor() }
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

fn crate_path_tokens(path: &CanonicalPath) -> TokenStream {
    let mut segments = path.segments().iter();
    segments
        .next()
        .expect("a canonical path has at least one segment");
    let rest = segments.map(|segment| format_ident!("{}", segment));

    quote! { crate #(:: #rest)* }
}
