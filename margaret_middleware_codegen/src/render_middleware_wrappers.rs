use heck::ToSnakeCase;
use proc_macro2::Ident;
use proc_macro2::TokenStream;
use quote::format_ident;
use quote::quote;

use margaret_attributes::name_allocator::NameAllocator;
use margaret_codegen_tokens::path_tokens::path_tokens;
use margaret_generated_module::generated_module_tokens::GeneratedModuleTokens;
use margaret_request_binding_codegen::binding_reads_request::binding_reads_request;
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
    let MiddlewarePlan {
        concrete,
        injections,
        parameters,
        wrapper,
        ..
    } = plan;
    let concrete = path_tokens(concrete);
    let routes_field = injections
        .routes
        .then(|| quote! { pub routes: std::sync::Arc<super::super::routes::Routes>, });
    let views_field = injections
        .views
        .then(|| quote! { pub views: std::sync::Arc<super::super::views::Views>, });

    let mut allocator = NameAllocator::new();

    for parameter in parameters {
        allocator.reserve(&parameter.holder.to_string());
    }

    let request_binding = if parameters
        .iter()
        .any(|parameter| binding_reads_request(&parameter.binding))
    {
        format_ident!("{}", allocator.allocate("request").field())
    } else {
        format_ident!("_request")
    };

    let next_binding = if parameters
        .iter()
        .any(|parameter| matches!(parameter.binding, RequestBinding::Next))
    {
        format_ident!("{}", allocator.allocate("next").field())
    } else {
        format_ident!("_next")
    };

    let continuation_return = quote! { return ::std::result::Result::Ok(response) };
    let error_return = quote! {
        return ::std::result::Result::Err(
            margaret::framework::http::handler_error::HandlerError::consumer(error),
        )
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
            },
        )
    });
    let call_arguments = parameters
        .iter()
        .map(|parameter| middleware_argument_value(parameter, &next_binding));
    let process_call = if plan.is_async {
        quote! { self.inner.process(#(#call_arguments),*).await }
    } else {
        quote! { self.inner.process(#(#call_arguments),*) }
    };

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
            ) -> ::std::result::Result<
                margaret::framework::http::response_continuation::ResponseContinuation,
                margaret::framework::http::handler_error::HandlerError,
            > {
                #(#extractions)*
                #process_call
                    .map_err(margaret::framework::http::handler_error::HandlerError::consumer)
            }
        }
    }
}

#[must_use]
pub fn render_middleware_wrappers(plans: &[MiddlewarePlan]) -> Vec<GeneratedModuleTokens> {
    let modules = plans.iter().map(|plan| {
        let wrapper = &plan.wrapper;
        format_ident!("{}", wrapper.to_string().to_snake_case())
    });
    let exports = plans.iter().map(|plan| {
        let wrapper = &plan.wrapper;
        let module = format_ident!("{}", wrapper.to_string().to_snake_case());

        quote! { pub use #module::#wrapper; }
    });
    let mut generated = vec![GeneratedModuleTokens::new(
        "middleware",
        quote! {
            #(mod #modules;)*
            #(#exports)*
        },
    )];

    generated.extend(plans.iter().map(|plan| {
        let wrapper = &plan.wrapper;
        let module = wrapper.to_string().to_snake_case();

        GeneratedModuleTokens::new(format!("middleware/{module}"), middleware_wrapper(plan))
    }));

    generated
}
