use proc_macro2::Ident;
use proc_macro2::TokenStream;
use quote::quote;

use margaret_codegen_tokens::grouped_integer_literal::grouped_integer_literal;

use crate::bound_parameter::BoundParameter;
use crate::content_binding::ContentBinding;
use crate::content_extraction_context::ContentExtractionContext;
use crate::render_model_extraction::render_model_extraction;
use crate::request_binding::RequestBinding;

fn render_models(
    parameters: &[BoundParameter],
    validation: &TokenStream,
    continuation_return: &TokenStream,
) -> TokenStream {
    let models = parameters
        .iter()
        .filter_map(|parameter| match &parameter.binding {
            RequestBinding::FormContent { extraction }
            | RequestBinding::JsonContent { extraction } => Some(render_model_extraction(
                extraction,
                &parameter.holder,
                validation,
                continuation_return,
            )),
            _ => None,
        });

    quote! { #(#models)* }
}

fn rejected_body(continuation_return: &TokenStream) -> TokenStream {
    quote! {
        {
            let response = margaret::framework::http::response_continuation::ResponseContinuation::from(
                rejection.into_response(),
            );

            #continuation_return
        }
    }
}

fn read_outcome(
    reading: &TokenStream,
    read_pattern: &TokenStream,
    continuation_return: &TokenStream,
) -> TokenStream {
    let rejected = rejected_body(continuation_return);

    quote! {
        match #reading {
            margaret::framework::http::body_reading::BodyReading::Read(#read_pattern) => #read_pattern,
            margaret::framework::http::body_reading::BodyReading::Rejected(rejection) => #rejected,
        }
    }
}

fn fallible_read_outcome(
    reading: &TokenStream,
    read_pattern: &TokenStream,
    context: &ContentExtractionContext,
) -> TokenStream {
    let rejected = rejected_body(context.continuation_return);
    let system_error_return = context.system_error_return;

    quote! {
        match #reading {
            ::std::result::Result::Ok(margaret::framework::http::body_reading::BodyReading::Read(#read_pattern)) => #read_pattern,
            ::std::result::Result::Ok(margaret::framework::http::body_reading::BodyReading::Rejected(rejection)) => #rejected,
            ::std::result::Result::Err(error) => #system_error_return,
        }
    }
}

fn reader_call(reader: &TokenStream, context: &ContentExtractionContext) -> TokenStream {
    let ContentExtractionContext {
        body_local,
        limit,
        request_local,
        ..
    } = context;
    let limit = grouped_integer_literal(*limit);

    quote! {
        #reader(
            #request_local,
            #body_local,
            margaret::framework::http::body_limit::BodyLimit::new(#limit),
        )
        .await
    }
}

fn render_modelled_content(
    reader: &TokenStream,
    validation: &TokenStream,
    parameters: &[BoundParameter],
    context: &ContentExtractionContext,
) -> TokenStream {
    let content_local = context.content_local;
    let read = read_outcome(
        &reader_call(reader, context),
        &quote! { #content_local },
        context.continuation_return,
    );
    let models = render_models(parameters, validation, context.continuation_return);

    quote! {
        let #content_local = #read;
        #models
    }
}

fn render_uploaded_files(files: &Ident, context: &ContentExtractionContext) -> TokenStream {
    let read = fallible_read_outcome(
        &reader_call(
            &quote! { margaret::framework::http::read_uploaded_files::read_uploaded_files },
            context,
        ),
        &quote! { files },
        context,
    );

    quote! { let #files = #read; }
}

fn render_fields_and_files(
    files: &Ident,
    parameters: &[BoundParameter],
    context: &ContentExtractionContext,
) -> TokenStream {
    let content_local = context.content_local;
    let read = fallible_read_outcome(
        &reader_call(
            &quote! { margaret::framework::http::read_multipart::read_multipart },
            context,
        ),
        &quote! { content },
        context,
    );
    let models = render_models(
        parameters,
        &quote! { margaret::framework::validation::validate::validate(&#content_local) },
        context.continuation_return,
    );

    quote! {
        let margaret::framework::http::multipart_content::MultipartContent {
            fields: #content_local,
            files: #files,
        } = #read;
        #models
    }
}

fn render_stream(stream: &Ident, context: &ContentExtractionContext) -> TokenStream {
    let body_local = context.body_local;
    let limit = grouped_integer_literal(context.limit);
    let read = read_outcome(
        &quote! {
            margaret::framework::http::request_body_stream::RequestBodyStream::open(
                #body_local,
                margaret::framework::http::body_limit::BodyLimit::new(#limit),
            )
        },
        &quote! { stream },
        context.continuation_return,
    );

    quote! { let #stream = #read; }
}

#[must_use]
pub fn render_content_extraction(
    binding: &ContentBinding,
    parameters: &[BoundParameter],
    context: &ContentExtractionContext,
) -> TokenStream {
    let content_local = context.content_local;

    match binding {
        ContentBinding::FormFields => render_modelled_content(
            &quote! { margaret::framework::http::read_form_fields::read_form_fields },
            &quote! { margaret::framework::validation::validate::validate(&#content_local) },
            parameters,
            context,
        ),
        ContentBinding::Json => render_modelled_content(
            &quote! { margaret::framework::http::read_json_value::read_json_value },
            &quote! { margaret::framework::validation::validate_json::validate_json(&#content_local) },
            parameters,
            context,
        ),
        ContentBinding::MultipartFiles { files } => render_uploaded_files(files, context),
        ContentBinding::MultipartFieldsAndFiles { files } => {
            render_fields_and_files(files, parameters, context)
        }
        ContentBinding::Stream { stream } => render_stream(stream, context),
    }
}
