use futures_util::TryFutureExt as _;
use uuid::Uuid;

use margaret::framework::active_record::change::Change;
use margaret::framework::active_record::json::Json;
use margaret::framework::active_record::lookup::Lookup;
use margaret::framework::active_record::model::Model;
use margaret::framework::database::database::Database;
use margaret::framework::macros::model;
use margaret_registered_claims::numeric_date::NumericDate;
use margaret_token_digest::token_digest::TokenDigest;

use crate::authorization_grant::AuthorizationGrant;
use crate::authorization_grants_error::AuthorizationGrantsError;
use crate::code_redemption::CodeRedemption;
use crate::issued_code::IssuedCode;

fn redeemed(
    AuthorizationCodeRecord {
        expires_at, grant, ..
    }: AuthorizationCodeRecord,
) -> CodeRedemption {
    CodeRedemption::Redeemed(Box::new(IssuedCode {
        expires_at: NumericDate::new(expires_at),
        grant: grant.into_payload(),
    }))
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

    /// Redeems the code for the family exactly once across every instance, reporting every later
    /// redemption with the family of the first one.
    ///
    /// # Errors
    ///
    /// Returns `AuthorizationGrantsError` when the code cannot be redeemed or its earlier
    /// redemption cannot be found.
    pub async fn redeem(
        database: &Database,
        code: TokenDigest,
        family: Uuid,
    ) -> Result<CodeRedemption, AuthorizationGrantsError> {
        let code = code.as_bytes().to_vec();

        Self::query()
            .code
            .eq(code.clone())
            .when(|stored| stored.redeemed_by.is_null())
            .update(database, |columns| columns.redeemed_by.to(Some(family)))
            .map_err(AuthorizationGrantsError::RedeemCode)
            .and_then(|change| async move {
                match change {
                    Change::Changed(record) => Ok(redeemed(record)),
                    Change::Unmatched => Self::query()
                        .code
                        .eq(code)
                        .find(database)
                        .await
                        .map(|lookup| match lookup {
                            Lookup::Found(Self {
                                redeemed_by: Some(family),
                                ..
                            }) => CodeRedemption::AlreadyRedeemed { family },
                            Lookup::Found(_) | Lookup::Missing => CodeRedemption::Unknown,
                        })
                        .map_err(AuthorizationGrantsError::FindRedeemedCode),
                }
            })
            .await
    }
}
