use proc_macro2::Ident;
use proc_macro2::TokenStream;
use quote::quote;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DeclaredIdTokenSigning {
    EllipticCurve,
    Rsa,
}

impl DeclaredIdTokenSigning {
    const ALL: [Self; 2] = [Self::EllipticCurve, Self::Rsa];

    pub(crate) fn of(keyword: &Ident) -> Option<Self> {
        Self::ALL
            .into_iter()
            .find(|signing| keyword == signing.keyword())
    }

    pub(crate) fn tokens(self) -> TokenStream {
        match self {
            Self::EllipticCurve => quote! {
                margaret::framework::jwks_secret_store::id_token_signing::IdTokenSigning::EllipticCurve
            },
            Self::Rsa => quote! {
                margaret::framework::jwks_secret_store::id_token_signing::IdTokenSigning::Rsa
            },
        }
    }

    fn keyword(self) -> &'static str {
        match self {
            Self::EllipticCurve => "elliptic_curve",
            Self::Rsa => "rsa",
        }
    }
}
