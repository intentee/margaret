use std::future::ready;

use futures_util::TryFutureExt as _;
use uuid::Uuid;

use margaret::framework::active_record::change::Change;
use margaret::framework::active_record::json::Json;
use margaret::framework::active_record::lookup::Lookup;
use margaret::framework::active_record::model::Model;
use margaret::framework::database::database::Database;
use margaret::framework::database::isolation::Isolation;
use margaret::framework::database::transaction::Transaction;
use margaret::framework::macros::model;
use margaret_registered_claims::numeric_date::NumericDate;
use margaret_token_digest::token_digest::TokenDigest;

use crate::authorization_grant::AuthorizationGrant;
use crate::authorization_grants_error::AuthorizationGrantsError;
use crate::code_redemption::CodeRedemption;
use crate::issued_code::IssuedCode;
use crate::redemption_decision::RedemptionDecision;
use crate::refresh_family_record::RefreshFamilyRecord;
use crate::sweep_refresh_families::sweep_refresh_families;

async fn earlier_redemption<Decided>(
    database: &Database,
    code: Vec<u8>,
) -> Result<CodeRedemption<Decided>, AuthorizationGrantsError> {
    AuthorizationCodeRecord::query()
        .code
        .eq(code)
        .find(database)
        .map_err(AuthorizationGrantsError::FindRedeemedCode)
        .await
        .map(|lookup| match lookup {
            Lookup::Found(AuthorizationCodeRecord {
                redeemed_by: Some(family),
                ..
            }) => CodeRedemption::AlreadyRedeemed { family },
            Lookup::Found(_) | Lookup::Missing => CodeRedemption::Unknown,
        })
}

async fn redeemed_within<Decided>(
    transaction: Transaction<'_>,
    database: &Database,
    code: Vec<u8>,
    family: Uuid,
    now: NumericDate,
    decide: impl FnOnce(AuthorizationGrant) -> RedemptionDecision<Decided>,
) -> Result<CodeRedemption<Decided>, AuthorizationGrantsError> {
    let redeemed = AuthorizationCodeRecord::query()
        .code
        .eq(code.clone())
        .when(|stored| stored.redeemed_by.is_null())
        .update(&transaction, |columns| columns.redeemed_by.to(Some(family)))
        .await
        .map_err(AuthorizationGrantsError::RedeemCode);

    ready(redeemed)
        .and_then(|change| async move {
            match change {
                Change::Changed(AuthorizationCodeRecord {
                    expires_at, grant, ..
                }) if expires_at > now.seconds_since_epoch() => {
                    settled(transaction, family, decide(grant.into_payload())).await
                }
                Change::Changed(_) => transaction
                    .commit()
                    .await
                    .map(|()| CodeRedemption::Unknown)
                    .map_err(AuthorizationGrantsError::CommitExpiredCodeRedemption),
                Change::Unmatched => {
                    let rolled_back = transaction
                        .rollback()
                        .await
                        .map_err(AuthorizationGrantsError::RollbackUnmatchedCodeRedemption);

                    ready(rolled_back)
                        .and_then(|()| earlier_redemption(database, code))
                        .await
                }
            }
        })
        .await
}

async fn settled<Decided>(
    transaction: Transaction<'_>,
    family: Uuid,
    decision: RedemptionDecision<Decided>,
) -> Result<CodeRedemption<Decided>, AuthorizationGrantsError> {
    match decision {
        RedemptionDecision::Consume(decided) => transaction
            .commit()
            .await
            .map(|()| CodeRedemption::Redeemed(decided))
            .map_err(AuthorizationGrantsError::CommitConsumedCodeRedemption),
        RedemptionDecision::OpenFamily { decided, opening } => {
            let opened = RefreshFamilyRecord::open(&transaction, family, opening).await;

            ready(opened)
                .and_then(|()| {
                    transaction
                        .commit()
                        .map_err(AuthorizationGrantsError::CommitRefreshFamilyOpening)
                })
                .await
                .map(|()| CodeRedemption::Redeemed(decided))
        }
    }
}

#[model(table = "authorization_codes")]
pub struct AuthorizationCodeRecord {
    #[column(primary_key, byte_length = 32)]
    pub code: Vec<u8>,
    #[column]
    #[index]
    pub expires_at: i64,
    #[column]
    pub grant: Json<AuthorizationGrant>,
    #[column]
    pub redeemed_by: Option<Uuid>,
}

impl AuthorizationCodeRecord {
    /// Holds the issued code until it expires, and forgets every code that expired at or before
    /// `now`.
    ///
    /// # Errors
    ///
    /// Returns `AuthorizationGrantsError` when the expired codes cannot be swept or the code
    /// cannot be issued.
    pub async fn issue(
        database: &Database,
        code: TokenDigest,
        IssuedCode { expires_at, grant }: IssuedCode,
        now: NumericDate,
    ) -> Result<(), AuthorizationGrantsError> {
        let issued = Self {
            code: code.as_bytes().to_vec(),
            expires_at: expires_at.seconds_since_epoch(),
            grant: Json::new(grant),
            redeemed_by: None,
        };

        Self::query()
            .expires_at
            .at_most(now.seconds_since_epoch())
            .delete(database)
            .map_err(AuthorizationGrantsError::SweepAuthorizationCodes)
            .and_then(|_swept| {
                issued
                    .insert()
                    .run(database)
                    .map_err(AuthorizationGrantsError::IssueCode)
            })
            .await
    }

    /// Redeems the code for the family exactly once across every instance, settling the
    /// redemption as `decide` concludes from its grant in the same transaction: either consuming
    /// the code alone, or opening the family with its first token too, so that every later
    /// redemption, reported with the family of the first one, finds the family already opened.
    /// Forgets every refresh family that expired at or before `now` first.
    ///
    /// # Errors
    ///
    /// Returns `AuthorizationGrantsError` when the expired families cannot be swept, the code
    /// cannot be redeemed, the family cannot be opened, or an earlier redemption cannot be found.
    pub async fn redeem<Decided>(
        database: &Database,
        code: TokenDigest,
        family: Uuid,
        now: NumericDate,
        decide: impl FnOnce(AuthorizationGrant) -> RedemptionDecision<Decided>,
    ) -> Result<CodeRedemption<Decided>, AuthorizationGrantsError> {
        let code = code.as_bytes().to_vec();

        sweep_refresh_families(database, now)
            .and_then(|()| {
                database
                    .connection()
                    .map_err(AuthorizationGrantsError::BeginCodeRedemption)
            })
            .and_then(|mut connection| async move {
                let began = connection
                    .transaction(Isolation::ReadCommitted)
                    .await
                    .map_err(AuthorizationGrantsError::BeginCodeRedemption);

                ready(began)
                    .and_then(|transaction| {
                        redeemed_within(transaction, database, code, family, now, decide)
                    })
                    .await
            })
            .await
    }
}
