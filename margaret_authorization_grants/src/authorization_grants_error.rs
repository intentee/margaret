use thiserror::Error;

use margaret::framework::active_record::active_record_error::ActiveRecordError;
use margaret::framework::database::database_error::DatabaseError;

#[derive(Debug, Error)]
pub enum AuthorizationGrantsError {
    #[error("the transaction opening a refresh family cannot begin: {0}")]
    BeginRefreshFamilyOpening(#[source] DatabaseError),

    #[error("the transaction rotating a refresh token cannot begin: {0}")]
    BeginRefreshTokenRotation(#[source] DatabaseError),

    #[error("the transaction opening a refresh family cannot commit: {0}")]
    CommitRefreshFamilyOpening(#[source] DatabaseError),

    #[error("the transaction rotating a refresh token cannot commit: {0}")]
    CommitRefreshTokenRotation(#[source] DatabaseError),

    #[error("the redeemed authorization code cannot be found: {0}")]
    FindRedeemedCode(#[source] ActiveRecordError),

    #[error("the revocation of the family of a refresh token cannot be found: {0}")]
    FindRefreshFamilyRevocation(#[source] ActiveRecordError),

    #[error("a refresh token cannot be found with its family: {0}")]
    FindRefreshToken(#[source] ActiveRecordError),

    #[error("the family of a locked refresh token cannot be found: {0}")]
    FindRotatedRefreshFamily(#[source] ActiveRecordError),

    #[error("the revocation of the family of a superseded refresh token cannot be found: {0}")]
    FindSupersededRefreshFamilyRevocation(#[source] ActiveRecordError),

    #[error("a superseded refresh token cannot be found: {0}")]
    FindSupersededRefreshToken(#[source] ActiveRecordError),

    #[error("a pending authorization cannot be held: {0}")]
    HoldPendingAuthorization(#[source] ActiveRecordError),

    #[error("an authorization code cannot be issued: {0}")]
    IssueCode(#[source] ActiveRecordError),

    #[error("the first refresh token of a family cannot be issued: {0}")]
    IssueFirstRefreshToken(#[source] ActiveRecordError),

    #[error("the next refresh token of a family cannot be issued: {0}")]
    IssueNextRefreshToken(#[source] ActiveRecordError),

    #[error("a refresh family cannot be opened: {0}")]
    OpenRefreshFamily(#[source] ActiveRecordError),

    #[error("an authorization code cannot be redeemed: {0}")]
    RedeemCode(#[source] ActiveRecordError),

    #[error("a refresh family cannot be revoked: {0}")]
    RevokeRefreshFamily(#[source] ActiveRecordError),

    #[error("the transaction rotating a refresh token of a closed family cannot roll back: {0}")]
    RollbackClosedRefreshTokenRotation(#[source] DatabaseError),

    #[error("the transaction opening a revoked refresh family cannot roll back: {0}")]
    RollbackRevokedRefreshFamilyOpening(#[source] DatabaseError),

    #[error("the transaction rotating a refresh token that is not current cannot roll back: {0}")]
    RollbackUnmatchedRefreshTokenRotation(#[source] DatabaseError),

    #[error("a refresh token cannot be rotated: {0}")]
    RotateRefreshToken(#[source] ActiveRecordError),

    #[error("the expired authorization codes cannot be swept: {0}")]
    SweepAuthorizationCodes(#[source] ActiveRecordError),

    #[error("the expired pending authorizations cannot be swept: {0}")]
    SweepPendingAuthorizations(#[source] ActiveRecordError),

    #[error("the expired refresh families cannot be swept: {0}")]
    SweepRefreshFamilies(#[source] ActiveRecordError),

    #[error("the expired refresh family revocations cannot be swept: {0}")]
    SweepRefreshFamilyRevocations(#[source] ActiveRecordError),

    #[error("a pending authorization cannot be taken: {0}")]
    TakePendingAuthorization(#[source] ActiveRecordError),
}
