use std::collections::BTreeSet;

use margaret_accepted_clients_codegen::declared_accepted_clients::DeclaredAcceptedClients;
use margaret_attributes::attribute_index::AttributeIndex;
use margaret_oauth_client_codegen::declared_client_registration::DeclaredClientRegistration;
use margaret_oauth_client_codegen::declared_oauth_clients::DeclaredOAuthClients;
use margaret_oauth_client_codegen::declared_sign_in::DeclaredSignIn;
use margaret_oauth_vocabulary::scope::Scope;
use margaret_oauth_vocabulary_codegen::declared_scopes::DeclaredScopes;
use margaret_sessions_codegen::declared_sessions::DeclaredSessions;
use margaret_tag_codegen::tag_pool::TagPool;
use margaret_token_issuance_codegen::declared_resource_issuances::DeclaredResourceIssuances;
use margaret_token_issuance_codegen::declared_token_issuance::DeclaredTokenIssuance;
use margaret_trusted_issuer_codegen::declared_trusts::DeclaredTrusts;

use crate::codegen_error::CodegenError;

fn referenced_scopes<'declarations>(
    oauth_clients: &'declarations DeclaredOAuthClients,
    accepted_clients: &'declarations DeclaredAcceptedClients,
) -> BTreeSet<&'declarations str> {
    let mut referenced: BTreeSet<&str> = accepted_clients
        .clients
        .iter()
        .flat_map(|client| client.scopes())
        .collect();

    for client in &oauth_clients.clients {
        if let DeclaredClientRegistration::External {
            sign_in: DeclaredSignIn::Declared { scopes, .. },
            ..
        } = &client.registration
        {
            referenced.extend(scopes.iter().map(Scope::as_str));
        }
    }

    referenced
}

pub(crate) struct IdentityDeclarations<'index> {
    pub(crate) accepted_clients: DeclaredAcceptedClients<'index>,
    pub(crate) oauth_clients: DeclaredOAuthClients<'index>,
    pub(crate) resource_issuances: DeclaredResourceIssuances<'index>,
    pub(crate) scopes: DeclaredScopes<'index>,
    pub(crate) sessions: DeclaredSessions<'index>,
    pub(crate) token_issuance: DeclaredTokenIssuance<'index>,
    pub(crate) trusts: DeclaredTrusts<'index>,
}

impl<'index> IdentityDeclarations<'index> {
    pub(crate) fn read(index: &'index AttributeIndex) -> Result<Self, CodegenError> {
        let token_issuance = DeclaredTokenIssuance::read(index)?;
        let trusts = DeclaredTrusts::read(index)?;
        let resource_issuances = DeclaredResourceIssuances::read(index, &token_issuance)?;
        let scopes = DeclaredScopes::read(index)?;
        let oauth_clients = DeclaredOAuthClients::read(index, &scopes)?;
        let accepted_clients =
            DeclaredAcceptedClients::read(index, &token_issuance, &resource_issuances, &scopes)?;
        let sessions = DeclaredSessions::read(index, &token_issuance, &resource_issuances)?;

        scopes.reject_unconsumed(&referenced_scopes(&oauth_clients, &accepted_clients))?;

        Ok(Self {
            accepted_clients,
            oauth_clients,
            resource_issuances,
            scopes,
            sessions,
            token_issuance,
            trusts,
        })
    }

    pub(crate) fn tags(
        &self,
        index: &'index AttributeIndex,
    ) -> Result<TagPool<'index>, CodegenError> {
        Ok(TagPool::collect(
            index,
            &self.trusts,
            &self.oauth_clients,
            &self.token_issuance,
            &self.resource_issuances,
            &self.accepted_clients,
            &self.sessions,
        )?)
    }
}
