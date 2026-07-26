use proc_macro2::TokenStream;
use quote::format_ident;
use quote::quote;

use margaret_attributes::name_allocator::NameAllocator;
use margaret_codegen_tokens::path_tokens::path_tokens;

use crate::authenticated_user_application::AuthenticatedUserApplication;
use crate::authenticated_user_login_route::AuthenticatedUserLoginRoute;
use crate::authenticated_user_provider::AuthenticatedUserProvider;
use crate::authenticated_user_wrapper_binding::AuthenticatedUserWrapperBinding;
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

fn provider_wrapper(binding: &AuthenticatedUserWrapperBinding<'_>) -> TokenStream {
    let AuthenticatedUserWrapperBinding {
        login_route,
        provider,
    } = binding;
    let AuthenticatedUserProvider {
        application,
        method_name,
        parameters,
    } = provider;
    let AuthenticatedUserApplication {
        concrete,
        injects_views,
        model,
        wrapper,
        ..
    } = application;
    let AuthenticatedUserLoginRoute { route, server } = login_route;
    let concrete = path_tokens(concrete);
    let model = path_tokens(model);
    let login_server = format_ident!("{}", server);
    let login_route = format_ident!("{}", route);
    let views_field =
        injects_views.then(|| quote! { pub views: std::sync::Arc<super::views::Views>, });

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

    let extraction_return = quote! {
        {
            return ::std::result::Result::Ok(
                margaret::framework::identity::authenticated_user_outcome::AuthenticatedUserOutcome::Rejected(response),
            );
        }
    };
    let provider_access = TokenStream::new();
    let extractions = parameters.iter().map(|parameter| {
        render_request_extraction(
            &parameter.binding,
            &parameter.holder,
            &ExtractionContext {
                continuation_return: &extraction_return,
                provider_access: &provider_access,
                request_local: &request_binding,
                response_return: &extraction_return,
            },
        )
    });
    let call_arguments = parameters.iter().map(provider_argument_value);

    quote! {
        pub struct #wrapper {
            pub inner: std::sync::Arc<#concrete>,
            pub routes: std::sync::Arc<super::routes::Routes>,
            #views_field
        }

        #[async_trait::async_trait]
        impl margaret::framework::identity::infers_authenticated_user::InfersAuthenticatedUser for #wrapper {
            type User = #model;

            async fn infer(
                &self,
                #request_binding: &margaret::framework::http::request::Request,
            ) -> ::anyhow::Result<margaret::framework::identity::authenticated_user_outcome::AuthenticatedUserOutcome<Self::User>> {
                #(#extractions)*

                ::std::result::Result::Ok(
                    match self.inner.#method_name(#(#call_arguments),*).await? {
                        margaret::framework::identity::authenticated_user_inference::AuthenticatedUserInference::Anonymous => {
                            margaret::framework::identity::authenticated_user_outcome::AuthenticatedUserOutcome::Anonymous
                        }
                        margaret::framework::identity::authenticated_user_inference::AuthenticatedUserInference::Authenticated(user) => {
                            margaret::framework::identity::authenticated_user_outcome::AuthenticatedUserOutcome::Authenticated(user)
                        }
                        margaret::framework::identity::authenticated_user_inference::AuthenticatedUserInference::LoginRequired => {
                            margaret::framework::identity::authenticated_user_outcome::AuthenticatedUserOutcome::LoginPageRedirect(
                                self.routes.#login_server.#login_route.see_other(),
                            )
                        }
                    },
                )
            }
        }
    }
}

#[must_use]
pub fn render_authenticated_user_wrappers(
    bindings: &[AuthenticatedUserWrapperBinding<'_>],
) -> TokenStream {
    let wrappers = bindings.iter().map(provider_wrapper);

    quote! {
        #(#wrappers)*
    }
}
