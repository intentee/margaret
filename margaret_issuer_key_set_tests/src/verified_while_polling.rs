use std::sync::Arc;

use tokio::sync::Notify;

use margaret_issuer_key_set::issuer_key_set::IssuerKeySet;
use margaret_jws_verification::verification_key_set::VerificationKeySet;
use margaret_jwt_verification::attributed_jwt::AttributedJwt;

use crate::fixture_now::fixture_now;
use crate::polled_verification::PolledVerification;
use crate::refresh_observation::RefreshObservation;

pub async fn verified_while_polling(
    issuer_key_set: &IssuerKeySet,
    attributed: &AttributedJwt<'_>,
    refreshed: Arc<VerificationKeySet>,
) -> PolledVerification {
    let verification_settled = Notify::new();
    let (verification, refresh) = tokio::join!(
        async {
            let verification = issuer_key_set.verify(attributed, fixture_now()).await;

            verification_settled.notify_one();

            verification
        },
        async {
            tokio::select! {
                () = issuer_key_set.refresh_requested() => {
                    issuer_key_set.start_fetch();
                    issuer_key_set.hold(refreshed);

                    RefreshObservation::Served
                }
                () = verification_settled.notified() => RefreshObservation::NotRequested,
            }
        }
    );

    PolledVerification {
        refresh,
        verification,
    }
}
