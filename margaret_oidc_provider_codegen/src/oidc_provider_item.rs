use proc_macro2::TokenStream;
use quote::quote;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum OidcProviderItem {
    AcceptedClients,
    AuthorizationEndpoint,
    ConsentEndpoint,
    ConsentHandler,
    IntrospectionEndpoint,
    IssuerMetadata,
    ProviderMetadataHandler,
    RevocationEndpoint,
    SubjectTokenExchangers,
    TokenEndpoint,
    UserinfoEndpoint,
}

impl OidcProviderItem {
    pub const ALL: [Self; 11] = [
        Self::AcceptedClients,
        Self::AuthorizationEndpoint,
        Self::ConsentEndpoint,
        Self::ConsentHandler,
        Self::IntrospectionEndpoint,
        Self::IssuerMetadata,
        Self::ProviderMetadataHandler,
        Self::RevocationEndpoint,
        Self::SubjectTokenExchangers,
        Self::TokenEndpoint,
        Self::UserinfoEndpoint,
    ];

    #[must_use]
    pub fn framework_path(self) -> TokenStream {
        match self {
            Self::AcceptedClients => quote! {
                margaret::framework::accepted_clients::accepted_clients::AcceptedClients
            },
            Self::AuthorizationEndpoint => quote! {
                margaret::framework::oidc_provider::authorization_endpoint::AuthorizationEndpoint
            },
            Self::ConsentEndpoint => quote! {
                margaret::framework::oidc_provider::consent_endpoint::ConsentEndpoint
            },
            Self::ConsentHandler => quote! {
                margaret::framework::oidc_provider::consent_handler::ConsentHandler
            },
            Self::IntrospectionEndpoint => quote! {
                margaret::framework::oidc_provider::introspection_endpoint::IntrospectionEndpoint
            },
            Self::IssuerMetadata => quote! {
                margaret::framework::issuer_metadata::issuer_metadata::IssuerMetadata
            },
            Self::ProviderMetadataHandler => quote! {
                margaret::framework::oidc_provider::provider_metadata_handler::ProviderMetadataHandler
            },
            Self::RevocationEndpoint => quote! {
                margaret::framework::oidc_provider::revocation_endpoint::RevocationEndpoint
            },
            Self::SubjectTokenExchangers => quote! {
                margaret::framework::subject_token_exchange::subject_token_exchangers::SubjectTokenExchangers
            },
            Self::TokenEndpoint => quote! {
                margaret::framework::oidc_provider::token_endpoint::TokenEndpoint
            },
            Self::UserinfoEndpoint => quote! {
                margaret::framework::oidc_provider::userinfo_endpoint::UserinfoEndpoint
            },
        }
    }

    #[must_use]
    pub fn uses_provider_endpoints(self) -> bool {
        match self {
            Self::AuthorizationEndpoint | Self::IssuerMetadata | Self::ProviderMetadataHandler => {
                true
            }
            Self::AcceptedClients
            | Self::ConsentEndpoint
            | Self::ConsentHandler
            | Self::IntrospectionEndpoint
            | Self::RevocationEndpoint
            | Self::SubjectTokenExchangers
            | Self::TokenEndpoint
            | Self::UserinfoEndpoint => false,
        }
    }

    #[must_use]
    pub fn type_name(self) -> &'static str {
        match self {
            Self::AcceptedClients => "AcceptedClients",
            Self::AuthorizationEndpoint => "AuthorizationEndpoint",
            Self::ConsentEndpoint => "ConsentEndpoint",
            Self::ConsentHandler => "ConsentHandler",
            Self::IntrospectionEndpoint => "IntrospectionEndpoint",
            Self::IssuerMetadata => "IssuerMetadata",
            Self::ProviderMetadataHandler => "ProviderMetadataHandler",
            Self::RevocationEndpoint => "RevocationEndpoint",
            Self::SubjectTokenExchangers => "SubjectTokenExchangers",
            Self::TokenEndpoint => "TokenEndpoint",
            Self::UserinfoEndpoint => "UserinfoEndpoint",
        }
    }
}
