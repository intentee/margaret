use std::ops::ControlFlow;

use url::Url;

use margaret_authorization_grants::authorization_grant::AuthorizationGrant;
use margaret_oauth_vocabulary::code_verifier::CodeVerifier;
use margaret_oauth_vocabulary::code_verifier_parsing::CodeVerifierParsing;

use crate::code_refusal::CodeRefusal;

pub(crate) struct CodeAdmission<'admission> {
    pub(crate) client_id: &'admission str,
    pub(crate) code_verifier: &'admission str,
    pub(crate) redirect_uri: &'admission Url,
}

impl CodeAdmission<'_> {
    pub(crate) fn admission(&self, grant: &AuthorizationGrant) -> ControlFlow<CodeRefusal> {
        match CodeVerifier::parse(self.code_verifier) {
            CodeVerifierParsing::Rejected(rejection) => {
                ControlFlow::Break(CodeRefusal::MalformedVerifier(rejection))
            }
            CodeVerifierParsing::Accepted(verifier)
                if grant.client_id == self.client_id
                    && grant.redirect_uri == *self.redirect_uri
                    && grant.code_challenge.admits(&verifier) =>
            {
                ControlFlow::Continue(())
            }
            CodeVerifierParsing::Accepted(_) => ControlFlow::Break(CodeRefusal::Mismatch),
        }
    }
}
