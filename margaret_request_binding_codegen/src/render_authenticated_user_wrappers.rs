use heck::ToSnakeCase;
use proc_macro2::Ident;
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
        challenge,
        concrete,
        injects_routes,
        injects_views,
        wrapper,
        ..
    }: &AuthenticatedUserApplication,
) -> TokenStream {
    let concrete = path_tokens(concrete);
    let routes_field = injects_routes
        .then(|| quote! { pub routes: std::sync::Arc<super::super::routes::Routes>, });
    let views_field =
        injects_views.then(|| quote! { pub views: std::sync::Arc<super::super::views::Views>, });
    let challenge_fields = match challenge {
        AuthenticatedUserChallenge::Bearer { trusted_issuers } => {
            let fields = trusted_issuers.iter().map(|trusted_issuer| {
                let field = format_ident!("{}", trusted_issuer.field);

                quote! {
                    pub #field: std::sync::Arc<margaret::framework::trusted_issuer::trusted_issuer::TrustedIssuer>,
                }
            });

            quote! { #(#fields)* }
        }
        AuthenticatedUserChallenge::Introspection {
            authorization_server,
        } => {
            let field = format_ident!("{}", authorization_server.field);

            quote! {
                pub #field: std::sync::Arc<margaret::framework::authorization_server_client::authorization_server_client::AuthorizationServerClient>,
            }
        }
        AuthenticatedUserChallenge::Unchallenged => TokenStream::new(),
    };

    quote! {
        pub struct #wrapper {
            pub inner: std::sync::Arc<#concrete>,
            #routes_field
            #views_field
            #challenge_fields
        }
    }
}

fn bearer_token_routing(
    challenge: &AuthenticatedUserChallenge,
    routed: &Ident,
    context: &ExtractionContext,
) -> TokenStream {
    let AuthenticatedUserChallenge::Bearer { trusted_issuers } = challenge else {
        return TokenStream::new();
    };
    let trusted_issuers = trusted_issuers.iter().map(|trusted_issuer| {
        let field = format_ident!("{}", trusted_issuer.field);

        quote! { self.#field.as_ref() }
    });
    let request = context.request_local;
    let continuation_return = context.continuation_return;
    let system_error_return = context.error_return;

    quote! {
        let #routed = match margaret::framework::bearer_token_verification::route_bearer_token::route_bearer_token(
            #request.inputs.server.authorization(),
            &[#(#trusted_issuers),*],
        )
        .map_err(margaret::framework::anyhow::Error::from)
        {
            ::std::result::Result::Ok(margaret::framework::bearer_token_verification::bearer_token_routing::BearerTokenRouting::Refused(response)) => #continuation_return,
            ::std::result::Result::Ok(margaret::framework::bearer_token_verification::bearer_token_routing::BearerTokenRouting::Routed(routed)) => routed,
            ::std::result::Result::Err(error) => #system_error_return,
        };
    }
}

fn parameter_extraction(
    parameter: &BoundParameter,
    routed: &Ident,
    context: &ExtractionContext,
) -> TokenStream {
    let RequestBinding::BearerToken {
        claims,
        profile,
        trusted_issuer,
    } = &parameter.binding
    else {
        return introspected_token_extraction(parameter, context);
    };
    let holder = &parameter.holder;
    let claims = path_tokens(claims);
    let profile = path_tokens(profile);
    let field = format_ident!("{}", trusted_issuer.field);
    let continuation_return = context.continuation_return;

    quote! {
        let #holder = match #routed
            .admit::<#claims, #profile>(self.#field.as_ref())
            .await
        {
            margaret::framework::bearer_token_verification::bearer_token_admission::BearerTokenAdmission::Admitted(token) => ::std::option::Option::Some(token),
            margaret::framework::bearer_token_verification::bearer_token_admission::BearerTokenAdmission::Refused(response) => #continuation_return,
            margaret::framework::bearer_token_verification::bearer_token_admission::BearerTokenAdmission::Unaddressed => ::std::option::Option::None,
        };
    }
}

fn introspected_token_extraction(
    parameter: &BoundParameter,
    context: &ExtractionContext,
) -> TokenStream {
    let RequestBinding::IntrospectedBearerToken {
        authorization_server,
        claims,
    } = &parameter.binding
    else {
        return render_request_extraction(&parameter.binding, &parameter.holder, context);
    };
    let holder = &parameter.holder;
    let claims = path_tokens(claims);
    let field = format_ident!("{}", authorization_server.field);
    let request = context.request_local;
    let continuation_return = context.continuation_return;
    let system_error_return = context.error_return;

    quote! {
        let #holder = match margaret::framework::token_introspection::introspect_bearer_token::introspect_bearer_token::<#claims>(
            #request.inputs.server.authorization(),
            self.#field.as_ref(),
        )
        .await
        .map_err(margaret::framework::anyhow::Error::from)
        {
            ::std::result::Result::Ok(margaret::framework::token_introspection::introspection_admission::IntrospectionAdmission::Admitted(token)) => ::std::option::Option::Some(token),
            ::std::result::Result::Ok(margaret::framework::token_introspection::introspection_admission::IntrospectionAdmission::Refused(response)) => #continuation_return,
            ::std::result::Result::Ok(margaret::framework::token_introspection::introspection_admission::IntrospectionAdmission::Unaddressed) => ::std::option::Option::None,
            ::std::result::Result::Err(error) => #system_error_return,
        };
    }
}

fn provider_wrapper(provider: &AuthenticatedUserProvider) -> TokenStream {
    let AuthenticatedUserProvider {
        application,
        is_async,
        method_name,
        parameters,
    } = provider;
    let AuthenticatedUserApplication { model, wrapper, .. } = application;
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

    let routed = format_ident!("{}", allocator.allocate("bearer_token").field());
    let continuation_return = quote! {
        return ::std::result::Result::Ok(
            margaret::framework::identity::authenticated_user_outcome::AuthenticatedUserOutcome::Interrupted(
                response,
            ),
        )
    };
    let error_return = quote! { return ::std::result::Result::Err(error) };
    let provider_access = TokenStream::new();
    let context = ExtractionContext {
        continuation_return: &continuation_return,
        error_return: &error_return,
        provider_access: &provider_access,
        request_local: &request_binding,
    };
    let routing = bearer_token_routing(&application.challenge, &routed, &context);
    let extractions = parameters
        .iter()
        .map(|parameter| parameter_extraction(parameter, &routed, &context));
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
                #routing
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
