use proc_macro2::TokenStream;
use quote::format_ident;
use quote::quote;

use margaret_codegen_tokens::path_tokens::path_tokens;

use crate::middleware_argument::MiddlewareArgument;
use crate::middleware_plan::MiddlewarePlan;

fn middleware_wrapper(plan: &MiddlewarePlan) -> TokenStream {
    let MiddlewarePlan {
        arguments,
        concrete,
        injects_routes,
        wrapper,
        ..
    } = plan;
    let concrete = path_tokens(concrete);
    let routes_field =
        injects_routes.then(|| quote! { pub routes: std::sync::Arc<super::routes::Routes>, });
    let request_binding = if arguments
        .iter()
        .any(|argument| matches!(argument, MiddlewareArgument::CurrentRequest))
    {
        format_ident!("request")
    } else {
        format_ident!("_request")
    };
    let next_binding = if arguments
        .iter()
        .any(|argument| matches!(argument, MiddlewareArgument::Next))
    {
        format_ident!("next")
    } else {
        format_ident!("_next")
    };
    let call_arguments = arguments.iter().map(|argument| match argument {
        MiddlewareArgument::CurrentRequest => quote! { request },
        MiddlewareArgument::Next => quote! { next },
        MiddlewareArgument::Routes => quote! { &self.routes },
    });

    quote! {
        pub struct #wrapper {
            pub inner: std::sync::Arc<#concrete>,
            #routes_field
        }

        #[async_trait::async_trait]
        impl margaret_http::http_middleware::HttpMiddleware for #wrapper {
            async fn process(
                &self,
                #request_binding: &margaret_http::request::Request,
                #next_binding: margaret_http::next::Next,
            ) -> margaret_http::response_continuation::ResponseContinuation {
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
