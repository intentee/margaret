use async_trait::async_trait;
use tokio::sync::Mutex;
use uuid::Uuid;

use margaret_authorization_grants::code_redemption::CodeRedemption;
use margaret_authorization_grants::family_opening::FamilyOpening;
use margaret_authorization_grants::issued_code::IssuedCode;
use margaret_authorization_grants::pending_authorization::PendingAuthorization;
use margaret_authorization_grants::pending_authorization_take::PendingAuthorizationTake;
use margaret_authorization_grants::refresh_family::RefreshFamily;
use margaret_authorization_grants::refresh_rotation::RefreshRotation;
use margaret_authorization_grants::refresh_token_lookup::RefreshTokenLookup;
use margaret_authorization_grants::stores_authorization_grants::StoresAuthorizationGrants;
use margaret_registered_claims::numeric_date::NumericDate;
use margaret_token_digest::token_digest::TokenDigest;

use crate::fixture_code::FixtureCode;
use crate::fixture_family::FixtureFamily;
use crate::fixture_grant_records::FixtureGrantRecords;
use crate::fixture_refresh_token::FixtureRefreshToken;
use crate::grant_interference::GrantInterference;
use crate::grant_operation::GrantOperation;
use crate::grant_storage_error::GrantStorageError;

pub struct FixtureAuthorizationGrants {
    interference: GrantInterference,
    records: Mutex<FixtureGrantRecords>,
}

impl FixtureAuthorizationGrants {
    #[must_use]
    pub fn interfered(interference: GrantInterference) -> Self {
        Self {
            interference,
            records: Mutex::new(FixtureGrantRecords::default()),
        }
    }

    #[must_use]
    pub fn undisturbed() -> Self {
        Self::interfered(GrantInterference::Undisturbed)
    }

    fn reached(&self, operation: GrantOperation) -> anyhow::Result<()> {
        match self.interference {
            GrantInterference::Failing(failing) if failing == operation => {
                Err(GrantStorageError::Unreachable { operation }.into())
            }
            GrantInterference::Failing(_)
            | GrantInterference::ReplayBeforeFamilyOpens
            | GrantInterference::RevocationBeforeRefreshRotates
            | GrantInterference::RotationBeforeRefreshRotates
            | GrantInterference::Undisturbed => Ok(()),
        }
    }
}

#[async_trait]
impl StoresAuthorizationGrants for FixtureAuthorizationGrants {
    async fn find_refresh_token(&self, token: TokenDigest) -> anyhow::Result<RefreshTokenLookup> {
        self.reached(GrantOperation::FindRefreshToken)?;

        let records = self.records.lock().await;

        Ok(match records.tokens.get(&token) {
            Some(held) => match records.families.get(&held.family) {
                Some(FixtureFamily::Open(record)) if held.current => RefreshTokenLookup::Current {
                    family: held.family,
                    record: record.clone(),
                },
                Some(FixtureFamily::Open(_)) => RefreshTokenLookup::Superseded {
                    family: held.family,
                },
                Some(FixtureFamily::Revoked { .. }) | None => RefreshTokenLookup::Unknown,
            },
            None => RefreshTokenLookup::Unknown,
        })
    }

    async fn hold_pending_authorization(
        &self,
        id: Uuid,
        pending: PendingAuthorization,
        now: NumericDate,
    ) -> anyhow::Result<()> {
        self.reached(GrantOperation::HoldPendingAuthorization)?;

        let mut records = self.records.lock().await;

        records.sweep_pending(now);
        records.pending.insert(id, pending);

        Ok(())
    }

    async fn issue_code(
        &self,
        code: TokenDigest,
        issued: IssuedCode,
        now: NumericDate,
    ) -> anyhow::Result<()> {
        self.reached(GrantOperation::IssueCode)?;

        let mut records = self.records.lock().await;

        records.sweep_codes(now);
        records
            .codes
            .insert(code, FixtureCode::Issued(Box::new(issued)));

        Ok(())
    }

    async fn open_refresh_family(
        &self,
        family: Uuid,
        record: RefreshFamily,
        first_token: TokenDigest,
        now: NumericDate,
    ) -> anyhow::Result<FamilyOpening> {
        self.reached(GrantOperation::OpenRefreshFamily)?;

        let mut records = self.records.lock().await;

        records.sweep_families(now);

        if let GrantInterference::ReplayBeforeFamilyOpens = self.interference {
            records.revoke(family, now);
        }

        Ok(match records.families.get(&family) {
            Some(FixtureFamily::Revoked { .. }) => FamilyOpening::Revoked,
            Some(FixtureFamily::Open(_)) | None => {
                records.families.insert(family, FixtureFamily::Open(record));
                records.tokens.insert(
                    first_token,
                    FixtureRefreshToken {
                        current: true,
                        family,
                    },
                );

                FamilyOpening::Opened
            }
        })
    }

    async fn redeem_code(&self, code: TokenDigest, family: Uuid) -> anyhow::Result<CodeRedemption> {
        self.reached(GrantOperation::RedeemCode)?;

        let mut records = self.records.lock().await;

        Ok(match records.codes.remove(&code) {
            Some(FixtureCode::Issued(issued)) => {
                records.codes.insert(
                    code,
                    FixtureCode::Redeemed {
                        expires_at: issued.expires_at,
                        family,
                    },
                );

                CodeRedemption::Redeemed(issued)
            }
            Some(FixtureCode::Redeemed {
                expires_at,
                family: redeemed,
            }) => {
                records.codes.insert(
                    code,
                    FixtureCode::Redeemed {
                        expires_at,
                        family: redeemed,
                    },
                );

                CodeRedemption::AlreadyRedeemed { family: redeemed }
            }
            None => CodeRedemption::Unknown,
        })
    }

    async fn revoke_refresh_family(&self, family: Uuid, now: NumericDate) -> anyhow::Result<()> {
        self.reached(GrantOperation::RevokeRefreshFamily)?;

        let mut records = self.records.lock().await;

        records.sweep_families(now);
        records.revoke(family, now);

        Ok(())
    }

    async fn rotate_refresh_token(
        &self,
        presented: TokenDigest,
        next: TokenDigest,
        now: NumericDate,
    ) -> anyhow::Result<RefreshRotation> {
        self.reached(GrantOperation::RotateRefreshToken)?;

        let mut guard = self.records.lock().await;
        let records = &mut *guard;

        records.sweep_families(now);

        if let GrantInterference::RevocationBeforeRefreshRotates = self.interference
            && let Some(held) = records.tokens.get(&presented)
        {
            records.revoke(held.family, now);
        }

        Ok(match records.open_family_of(&presented) {
            Some(&FixtureRefreshToken {
                current: true,
                family,
            }) => records.rotate_current(self.interference, presented, next, family),
            Some(&FixtureRefreshToken {
                current: false,
                family,
            }) => RefreshRotation::Superseded { family },
            None => RefreshRotation::Unknown,
        })
    }

    async fn take_pending_authorization(
        &self,
        id: Uuid,
    ) -> anyhow::Result<PendingAuthorizationTake> {
        self.reached(GrantOperation::TakePendingAuthorization)?;

        Ok(match self.records.lock().await.pending.remove(&id) {
            Some(pending) => PendingAuthorizationTake::Taken(Box::new(pending)),
            None => PendingAuthorizationTake::Absent,
        })
    }
}
