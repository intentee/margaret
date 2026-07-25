use proc_macro2::TokenStream;
use quote::format_ident;
use quote::quote;

use margaret_attributes::name_allocator::NameAllocator;
use margaret_codegen_tokens::path_tokens::path_tokens;

use crate::authenticated_user_application::AuthenticatedUserApplication;
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

fn provider_wrapper(provider: &AuthenticatedUserProvider) -> TokenStream {
    let AuthenticatedUserProvider {
        application,
        method_name,
        parameters,
    } = provider;
    let AuthenticatedUserApplication {
        concrete,
        injects_routes,
        injects_views,
        model,
        wrapper,
        ..
    } = application;
    let concrete = path_tokens(concrete);
    let model = path_tokens(model);
    let routes_field =
        injects_routes.then(|| quote! { pub routes: std::sync::Arc<super::routes::Routes>, });
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

    let continuation_return = quote! {
        return margaret::framework::identity::authenticated_user_outcome::AuthenticatedUserOutcome::Interrupted(
            response,
        )
    };
    let response_return = quote! {
        return margaret::framework::identity::authenticated_user_outcome::AuthenticatedUserOutcome::Interrupted(
            response.into(),
        )
    };
    let provider_access = TokenStream::new();
    let extractions = parameters.iter().map(|parameter| {
        render_request_extraction(
            &parameter.binding,
            &parameter.holder,
            &ExtractionContext {
                continuation_return: &continuation_return,
                provider_access: &provider_access,
                request_local: &request_binding,
                response_return: &response_return,
            },
        )
    });
    let call_arguments = parameters.iter().map(provider_argument_value);

    quote! {
        pub struct #wrapper {
            pub inner: std::sync::Arc<#concrete>,
            #routes_field
            #views_field
        }

        #[async_trait::async_trait]
        impl margaret::framework::identity::infers_authenticated_user::InfersAuthenticatedUser for #wrapper {
            type User = #model;

            async fn infer(
                &self,
                #request_binding: &margaret::framework::http::request::Request,
            ) -> margaret::framework::identity::authenticated_user_outcome::AuthenticatedUserOutcome<Self::User> {
                #(#extractions)*
                self.inner.#method_name(#(#call_arguments),*).await
            }
        }
    }
}

#[must_use]
pub fn render_authenticated_user_wrappers(providers: &[&AuthenticatedUserProvider]) -> TokenStream {
    let wrappers = providers.iter().copied().map(provider_wrapper);

    quote! {
        #(#wrappers)*
    }
}
