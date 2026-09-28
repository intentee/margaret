use heck::ToSnakeCase;
use proc_macro2::TokenStream;
use quote::format_ident;
use quote::quote;

use margaret_attributes::name_allocator::NameAllocator;
use margaret_codegen_tokens::path_tokens::path_tokens;
use margaret_generated_module::generated_module_tokens::GeneratedModuleTokens;

use crate::authenticated_user_application::AuthenticatedUserApplication;
use crate::authenticated_user_challenge::AuthenticatedUserChallenge;
use crate::authenticated_user_provider::AuthenticatedUserProvider;
use crate::binding_reads_request::binding_reads_request;
use crate::binding_shadows_request::binding_shadows_request;
use crate::bound_parameter::BoundParameter;
use crate::extraction_context::ExtractionContext;
use crate::render_oidc_token_extractions::render_oidc_token_extractions;
use crate::render_request_extraction::render_request_extraction;
use crate::request_binding::RequestBinding;

fn provider_argument_value(parameter: &BoundParameter) -> TokenStream {
    match &parameter.binding {
        RequestBinding::Routes => quote! { &self.routes },
        RequestBinding::Views => quote! { &self.views },
        _ => {
            let holder = &parameter.holder;

            quote! { #holder }
        }
    }
}

fn wrapper_struct(
    AuthenticatedUserApplication {
        concrete,
        injects_routes,
        injects_views,
        oidc_token_verifiers,
        wrapper,
        ..
    }: &AuthenticatedUserApplication,
) -> TokenStream {
    let concrete = path_tokens(concrete);
    let routes_field = injects_routes
        .then(|| quote! { pub routes: std::sync::Arc<super::super::routes::Routes>, });
    let views_field =
        injects_views.then(|| quote! { pub views: std::sync::Arc<super::super::views::Views>, });
    let verifier_fields = oidc_token_verifiers.iter().map(|verifier| {
        let field = format_ident!("{}", verifier.field);

        quote! {
            pub #field: std::sync::Arc<margaret::framework::oidc_client::oidc_token_verifier::OidcTokenVerifier>,
        }
    });

    quote! {
        pub struct #wrapper {
            pub inner: std::sync::Arc<#concrete>,
            #routes_field
            #views_field
            #(#verifier_fields)*
        }
    }
}

fn provider_wrapper(provider: &AuthenticatedUserProvider) -> TokenStream {
    let AuthenticatedUserProvider {
        application,
        is_async,
        method_name,
        parameters,
    } = provider;
    let AuthenticatedUserApplication {
        challenge,
        model,
        wrapper,
        ..
    } = application;
    let wrapper_struct = wrapper_struct(application);
    let model = path_tokens(model);
    let mut allocator = NameAllocator::new();

    for parameter in parameters {
        if binding_shadows_request(&parameter.binding) {
            allocator.reserve(&parameter.holder.to_string());
        }
    }

    let request_binding = if parameters
        .iter()
        .any(|parameter| binding_reads_request(&parameter.binding))
    {
        format_ident!("{}", allocator.allocate("request").field())
    } else {
        format_ident!("_request")
    };

    let continuation_return = quote! {
        return ::std::result::Result::Ok(
            margaret::framework::identity::authenticated_user_outcome::AuthenticatedUserOutcome::Interrupted(
                response,
            ),
        )
    };
    let response_return = quote! {
        return ::std::result::Result::Ok(
            margaret::framework::identity::authenticated_user_outcome::AuthenticatedUserOutcome::Interrupted(
                response.into(),
            ),
        )
    };
    let error_return = quote! { return ::std::result::Result::Err(error) };
    let oidc_token_extractions = match challenge {
        AuthenticatedUserChallenge::Bearer => render_oidc_token_extractions(
            parameters,
            &format_ident!("{}", allocator.allocate("presented_bearer").field()),
            &request_binding,
            &error_return,
        ),
        AuthenticatedUserChallenge::Unchallenged => TokenStream::new(),
    };
    let provider_access = TokenStream::new();
    let extractions = parameters.iter().map(|parameter| {
        render_request_extraction(
            &parameter.binding,
            &parameter.holder,
            &ExtractionContext {
                continuation_return: &continuation_return,
                error_return: &error_return,
                provider_access: &provider_access,
                request_local: &request_binding,
                response_return: &response_return,
            },
        )
    });
    let call_arguments = parameters.iter().map(provider_argument_value);
    let infer_call = if *is_async {
        quote! { self.inner.#method_name(#(#call_arguments),*).await }
    } else {
        quote! { self.inner.#method_name(#(#call_arguments),*) }
    };

    quote! {
        #wrapper_struct

        #[async_trait::async_trait]
        impl margaret::framework::identity::infers_authenticated_user::InfersAuthenticatedUser for #wrapper {
            type User = #model;

            async fn infer(
                &self,
                #request_binding: &margaret::framework::http::request::Request,
            ) -> margaret::framework::anyhow::Result<
                margaret::framework::identity::authenticated_user_outcome::AuthenticatedUserOutcome<Self::User>,
            > {
                #oidc_token_extractions
                #(#extractions)*
                #infer_call
            }
        }
    }
}

#[must_use]
pub fn render_authenticated_user_wrappers(
    providers: &[&AuthenticatedUserProvider],
) -> Vec<GeneratedModuleTokens> {
    let modules = providers.iter().map(|provider| {
        let wrapper = &provider.application.wrapper;
        format_ident!("{}", wrapper.to_string().to_snake_case())
    });
    let exports = providers.iter().map(|provider| {
        let wrapper = &provider.application.wrapper;
        let module = format_ident!("{}", wrapper.to_string().to_snake_case());

        quote! { pub use #module::#wrapper; }
    });
    let mut generated = vec![GeneratedModuleTokens::new(
        "authenticated_users",
        quote! {
            #(mod #modules;)*
            #(#exports)*
        },
    )];

    generated.extend(providers.iter().map(|provider| {
        let wrapper = &provider.application.wrapper;
        let module = wrapper.to_string().to_snake_case();

        GeneratedModuleTokens::new(
            format!("authenticated_users/{module}"),
            provider_wrapper(provider),
        )
    }));

    generated
}
