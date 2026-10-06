use std::iter;
use std::sync::Arc;

use futures_util::future::select_all;
use tokio::time::Instant;

use margaret_jws_verification::verification_key_set::VerificationKeySet;
use margaret_registered_claims::issuer_identifier::IssuerIdentifier;
use margaret_trusted_issuer::key_set_locator::KeySetLocator;
use margaret_trusted_issuer::trusted_issuer::TrustedIssuer;

use crate::group_locator::GroupLocator;
use crate::issuer_directory_error::IssuerDirectoryError;

pub(crate) struct IssuerGroup {
    pub(crate) locator: GroupLocator,
    lead: Arc<TrustedIssuer>,
    others: Vec<Arc<TrustedIssuer>>,
}

impl IssuerGroup {
    pub(crate) fn founded_by(trusted_issuer: Arc<TrustedIssuer>) -> Self {
        let locator = match &trusted_issuer.locator {
            KeySetLocator::Discovery {
                discovery_url,
                metadata,
            } => GroupLocator::Discovery {
                discovery_url: discovery_url.clone(),
                metadata: vec![Arc::clone(metadata)],
            },
            KeySetLocator::Endpoint(endpoint) => GroupLocator::Endpoint(Arc::clone(endpoint)),
        };

        Self {
            locator,
            lead: trusted_issuer,
            others: Vec::new(),
        }
    }

    pub(crate) fn admit(
        &mut self,
        trusted_issuer: Arc<TrustedIssuer>,
    ) -> Result<(), IssuerDirectoryError> {
        let trust = trusted_issuer.trust.token_trust();

        if self
            .members()
            .any(|member| member.trust.token_trust().audience == trust.audience)
        {
            return Err(IssuerDirectoryError::TokenTrustDeclaredTwice {
                audience: trust.audience.clone(),
                issuer: Box::new(trust.issuer.clone()),
            });
        }

        let trusted_twice = || IssuerDirectoryError::JwksEndpointIssuerTrustedTwice {
            issuer: trust.issuer.clone(),
        };
        let KeySetLocator::Discovery { metadata, .. } = &trusted_issuer.locator else {
            return Err(trusted_twice());
        };
        let GroupLocator::Discovery {
            metadata: shared, ..
        } = &mut self.locator
        else {
            return Err(trusted_twice());
        };

        shared.push(Arc::clone(metadata));
        self.others.push(trusted_issuer);

        Ok(())
    }

    pub(crate) fn fail_fetch(&self) {
        for trusted_issuer in self.members() {
            trusted_issuer.key_set.fail_fetch();
        }
    }

    pub(crate) fn hold(&self, key_set: &Arc<VerificationKeySet>) {
        for trusted_issuer in self.members() {
            trusted_issuer.key_set.hold(Arc::clone(key_set));
        }
    }

    pub(crate) fn issuer(&self) -> &IssuerIdentifier {
        &self.lead.trust.token_trust().issuer
    }

    pub(crate) fn next_fetch_due(&self, started_at: Instant) -> Instant {
        self.lead
            .key_set
            .snapshot()
            .holding
            .next_fetch_due(started_at)
    }

    pub(crate) async fn refresh_requested(&self) {
        select_all(
            self.members()
                .map(|trusted_issuer| Box::pin(trusted_issuer.key_set.refresh_requested())),
        )
        .await;
    }

    pub(crate) fn start_fetch(&self) {
        for trusted_issuer in self.members() {
            trusted_issuer.key_set.start_fetch();
        }
    }

    pub(crate) fn stop_polling(&self) {
        for trusted_issuer in self.members() {
            trusted_issuer.key_set.stop_polling();
        }
    }

    fn members(&self) -> impl Iterator<Item = &Arc<TrustedIssuer>> {
        iter::once(&self.lead).chain(&self.others)
    }
}
