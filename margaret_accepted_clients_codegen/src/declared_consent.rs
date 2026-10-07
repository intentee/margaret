use proc_macro2::Ident;
use proc_macro2::TokenStream;
use quote::quote;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DeclaredConsent {
    Implicit,
    Prompted,
}

impl DeclaredConsent {
    const ALL: [Self; 2] = [Self::Implicit, Self::Prompted];

    pub(crate) fn of(keyword: &Ident) -> Option<Self> {
        Self::ALL
            .into_iter()
            .find(|consent| keyword == consent.keyword())
    }

    pub(crate) fn tokens(self) -> TokenStream {
        match self {
            Self::Implicit => quote! {
                margaret::framework::accepted_clients::consent_policy::ConsentPolicy::Implicit
            },
            Self::Prompted => quote! {
                margaret::framework::accepted_clients::consent_policy::ConsentPolicy::Prompted
            },
        }
    }

    fn keyword(self) -> &'static str {
        match self {
            Self::Implicit => "implicit",
            Self::Prompted => "prompted",
        }
    }
}
