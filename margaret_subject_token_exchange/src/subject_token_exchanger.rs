use std::sync::Arc;

use margaret_jwt_verification::jwt_addressee::JwtAddressee;
use margaret_registered_claims::issuer_identifier::IssuerIdentifier;
use margaret_trusted_issuer::trusted_issuer::TrustedIssuer;

use crate::exchanges_presented_tokens::ExchangesPresentedTokens;
use crate::exchanges_subject_tokens::ExchangesSubjectTokens;
use crate::typed_exchange::TypedExchange;

pub struct SubjectTokenExchanger {
    pub(crate) exchange: Box<dyn ExchangesPresentedTokens>,
    pub(crate) trusted_issuer: Arc<TrustedIssuer>,
}

impl SubjectTokenExchanger {
    #[must_use]
    pub fn create<TExchanger: ExchangesSubjectTokens>(
        trusted_issuer: Arc<TrustedIssuer>,
        exchanger: Arc<TExchanger>,
    ) -> Self {
        Self {
            exchange: Box::new(TypedExchange { exchanger }),
            trusted_issuer,
        }
    }
}

impl JwtAddressee for SubjectTokenExchanger {
    fn jwt_issuer(&self) -> &IssuerIdentifier {
        self.trusted_issuer.jwt_issuer()
    }
}
