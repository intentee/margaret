use proc_macro2::Ident;
use proc_macro2::TokenStream;
use quote::format_ident;
use quote::quote;

use margaret_attributes::name_allocator::NameAllocator;

use crate::bound_parameter::BoundParameter;
use crate::captured_provider::CapturedProvider;
use crate::captured_provider_kind::CapturedProviderKind;
use crate::request_binding::RequestBinding;

fn provider_kind(binding: &RequestBinding) -> Option<CapturedProviderKind> {
    match binding {
        RequestBinding::AuthenticatedUser { application, .. } => {
            Some(CapturedProviderKind::AuthenticatedUser {
                application: application.clone(),
            })
        }
        RequestBinding::BoundRouteParameter {
            binder_field,
            binder_provider,
            ..
        } => Some(CapturedProviderKind::Binder {
            accessor: binder_field.clone(),
            provider: binder_provider.clone(),
        }),
        RequestBinding::AssetBag
        | RequestBinding::CurrentRequest
        | RequestBinding::FormRequest { .. }
        | RequestBinding::Forwarder
        | RequestBinding::Injectable { .. }
        | RequestBinding::Next
        | RequestBinding::PeerSpiffeId
        | RequestBinding::RouteParameterValue { .. }
        | RequestBinding::Routes
        | RequestBinding::Views => None,
    }
}

pub struct CapturedProviders {
    captured: Vec<CapturedProvider>,
}

impl CapturedProviders {
    #[must_use]
    pub fn capture(parameters: &[BoundParameter], allocator: &mut NameAllocator) -> Self {
        let mut captured: Vec<CapturedProvider> = Vec::new();

        for parameter in parameters {
            let Some(kind) = provider_kind(&parameter.binding) else {
                continue;
            };

            if captured
                .iter()
                .any(|entry| entry.kind.binds(&parameter.binding))
            {
                continue;
            }

            let local = format_ident!("{}", allocator.allocate(kind.accessor()).field());

            captured.push(CapturedProvider { kind, local });
        }

        Self { captured }
    }

    #[must_use]
    pub fn access(&self, binding: &RequestBinding, owner: &TokenStream) -> TokenStream {
        match self.local(binding) {
            Some(local) => quote! { #owner #local },
            None => TokenStream::new(),
        }
    }

    pub fn entries(&self) -> impl Iterator<Item = &CapturedProvider> {
        self.captured.iter()
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.captured.is_empty()
    }

    fn local(&self, binding: &RequestBinding) -> Option<&Ident> {
        self.captured
            .iter()
            .find(|entry| entry.kind.binds(binding))
            .map(|entry| &entry.local)
    }
}
