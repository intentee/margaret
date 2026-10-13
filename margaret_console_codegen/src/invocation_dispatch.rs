use proc_macro2::TokenStream;
use quote::quote;

#[derive(Clone, Copy)]
pub(crate) enum InvocationDispatch {
    Asynchronous,
    Synchronous,
}

impl InvocationDispatch {
    pub(crate) fn invocation_type(self) -> TokenStream {
        match self {
            Self::Asynchronous => {
                quote! { margaret::framework::console::command_future::CommandFuture }
            }
            Self::Synchronous => quote! {
                ::std::boxed::Box<
                    dyn ::std::ops::FnOnce()
                        -> margaret::framework::console::command_outcome::CommandOutcome,
                >
            },
        }
    }

    pub(crate) fn invoked(self) -> TokenStream {
        match self {
            Self::Asynchronous => quote! { command_run.await },
            Self::Synchronous => quote! { command_run() },
        }
    }

    pub(crate) fn run(self, body: &TokenStream) -> TokenStream {
        let command_run = match self {
            Self::Asynchronous => quote! {
                margaret::framework::console::command_future::CommandFuture::new(async move {
                    #body
                })
            },
            Self::Synchronous => quote! { ::std::boxed::Box::new(move || { #body }) },
        };

        quote! { self::invocation::Invocation::Run(#command_run) }
    }

    pub(crate) fn run_asyncness(self) -> TokenStream {
        match self {
            Self::Asynchronous => quote! { async },
            Self::Synchronous => quote! {},
        }
    }
}
