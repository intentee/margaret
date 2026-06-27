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
use crate::provided_type::ProvidedType;
use crate::provider::Provider;
use crate::provider_construction::ProviderConstruction;

pub(crate) fn render(plan: &ContainerPlan) -> String {
    let is_async = plan.providers.iter().any(Provider::is_async);
    let fields = plan
        .providers
        .iter()
        .map(|provider| field_declaration(provider, is_async));
    let initializers = plan
        .providers
        .iter()
        .map(|provider| field_initializer(provider, is_async));
    let accessors = plan
        .providers
        .iter()
        .map(|provider| accessor(provider, plan, is_async));

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

fn cell_type(is_async: bool) -> TokenStream {
    if is_async {
        quote! { tokio::sync::OnceCell }
    } else {
        quote! { std::sync::OnceLock }
    }
}

fn field_declaration(provider: &Provider, is_async: bool) -> TokenStream {
    let name = field_ident(provider);
    let field_type = field_type(provider);
    let cell = cell_type(is_async);

    quote! { #name: #cell<#field_type> }
}

fn field_initializer(provider: &Provider, is_async: bool) -> TokenStream {
    let name = field_ident(provider);
    let cell = cell_type(is_async);

    quote! { #name: #cell::new() }
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

fn accessor(provider: &Provider, plan: &ContainerPlan, is_async: bool) -> TokenStream {
    let name = field_ident(provider);
    let field_type = field_type(provider);
    let construction = construction(provider, plan, is_async);

    if is_async {
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
    } else {
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
}

fn construction(provider: &Provider, plan: &ContainerPlan, is_async: bool) -> TokenStream {
    match &provider.construction {
        ProviderConstruction::Direct(direct) => {
            let value = direct_value(direct, &provider.concrete_path, plan, is_async);

            quote! { std::sync::Arc::new(#value) }
        }
        ProviderConstruction::Factory {
            factory_is_async,
            factory_method,
            provider: own,
        } => {
            let value = direct_value(own, &provider.concrete_path, plan, is_async);
            let factory = format_ident!("{}", factory_method);
            let call = if *factory_is_async {
                quote! { provider.#factory().await }
            } else {
                quote! { provider.#factory() }
            };

            quote! {
                {
                    let provider = #value;

                    #call
                }
            }
        }
    }
}

fn direct_value(
    direct: &DirectConstruction,
    concrete_path: &CanonicalPath,
    plan: &ContainerPlan,
    is_async: bool,
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
                .map(|dependency| dependency_expression(dependency, plan, is_async));

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
    is_async: bool,
) -> TokenStream {
    match dependency {
        DependencyKind::Single { provider_key } => {
            let accessor = field_ident(provider_by_key(plan, provider_key));

            access(&accessor, is_async)
        }
        DependencyKind::Collection { trait_path } => {
            let elements = plan
                .collections
                .members_of(trait_path)
                .iter()
                .map(|member| {
                    let accessor = field_ident(provider_by_key(plan, member));

                    access(&accessor, is_async)
                });

            quote! { vec![#(#elements),*] }
        }
    }
}

fn access(accessor: &Ident, is_async: bool) -> TokenStream {
    if is_async {
        quote! { self.#accessor().await }
    } else {
        quote! { self.#accessor() }
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
