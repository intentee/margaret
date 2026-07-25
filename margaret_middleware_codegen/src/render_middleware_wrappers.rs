use proc_macro2::Ident;
use proc_macro2::TokenStream;
use quote::format_ident;
use quote::quote;

use margaret_attributes::name_allocator::NameAllocator;
use margaret_codegen_tokens::path_tokens::path_tokens;
use margaret_request_binding_codegen::binding_reads_request::binding_reads_request;
use margaret_request_binding_codegen::binding_shadows_request::binding_shadows_request;
use margaret_request_binding_codegen::bound_parameter::BoundParameter;
use margaret_request_binding_codegen::extraction_context::ExtractionContext;
use margaret_request_binding_codegen::render_request_extraction::render_request_extraction;
use margaret_request_binding_codegen::request_binding::RequestBinding;

use crate::middleware_plan::MiddlewarePlan;

fn middleware_argument_value(parameter: &BoundParameter, next_binding: &Ident) -> TokenStream {
    match &parameter.binding {
        RequestBinding::Next => quote! { #next_binding },
        RequestBinding::Routes => quote! { &self.routes },
        RequestBinding::Views => quote! { &self.views },
        _ => {
            let holder = &parameter.holder;

            quote! { #holder }
        }
    }
}

fn middleware_wrapper(plan: &MiddlewarePlan) -> TokenStream {
    let injects_routes = plan.injects_routes();
    let injects_views = plan.injects_views();
    let MiddlewarePlan {
        concrete,
        parameters,
        wrapper,
        ..
    } = plan;
    let concrete = path_tokens(concrete);
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

    for parameter in parameters {
        if matches!(parameter.binding, RequestBinding::CurrentRequest) {
            allocator.reserve(&parameter.holder.to_string());
        }
    }

    let next_binding = if parameters
        .iter()
        .any(|parameter| matches!(parameter.binding, RequestBinding::Next))
    {
        format_ident!("{}", allocator.allocate("next").field())
    } else {
        format_ident!("_next")
    };

    let continuation_return = quote! { return response };
    let response_return = quote! { return response.into() };
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
    let call_arguments = parameters
        .iter()
        .map(|parameter| middleware_argument_value(parameter, &next_binding));

    quote! {
        pub struct #wrapper {
            pub inner: std::sync::Arc<#concrete>,
            #routes_field
            #views_field
        }

        #[async_trait::async_trait]
        impl margaret::framework::http::http_middleware::HttpMiddleware for #wrapper {
            async fn process(
                &self,
                #request_binding: &margaret::framework::http::request::Request,
                #next_binding: margaret::framework::http::next::Next,
            ) -> margaret::framework::http::response_continuation::ResponseContinuation {
                #(#extractions)*
                self.inner.process(#(#call_arguments),*).await
            }
        }
    }
}

pub fn render_middleware_wrappers(plans: &[MiddlewarePlan]) -> TokenStream {
    let wrappers = plans.iter().map(middleware_wrapper);

    quote! {
        #(#wrappers)*
    }
}
