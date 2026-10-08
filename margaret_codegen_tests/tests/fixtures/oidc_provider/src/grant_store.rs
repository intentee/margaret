use async_trait::async_trait;
use uuid::Uuid;

use margaret::framework::authorization_grants::code_redemption::CodeRedemption;
use margaret::framework::authorization_grants::family_opening::FamilyOpening;
use margaret::framework::authorization_grants::issued_code::IssuedCode;
use margaret::framework::authorization_grants::pending_authorization::PendingAuthorization;
use margaret::framework::authorization_grants::pending_authorization_take::PendingAuthorizationTake;
use margaret::framework::authorization_grants::refresh_family::RefreshFamily;
use margaret::framework::authorization_grants::refresh_rotation::RefreshRotation;
use margaret::framework::authorization_grants::refresh_token_lookup::RefreshTokenLookup;
use margaret::framework::authorization_grants::stores_authorization_grants::StoresAuthorizationGrants;
use margaret::framework::macros::singleton;
use margaret::framework::macros::stores_authorization_grants;
use margaret::framework::token_digest::token_digest::TokenDigest;

#[singleton]
#[stores_authorization_grants]
pub struct GrantStore;

#[async_trait]
impl StoresAuthorizationGrants for GrantStore {
    async fn find_refresh_token(&self, _token: TokenDigest) -> anyhow::Result<RefreshTokenLookup> {
        Ok(RefreshTokenLookup::Unknown)
    }

    async fn hold_pending_authorization(
        &self,
        _id: Uuid,
        _pending: PendingAuthorization,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    async fn issue_code(&self, _code: TokenDigest, _issued: IssuedCode) -> anyhow::Result<()> {
        Ok(())
    }

    async fn open_refresh_family(
        &self,
        _family: Uuid,
        _record: RefreshFamily,
        _first_token: TokenDigest,
    ) -> anyhow::Result<FamilyOpening> {
        Ok(FamilyOpening::Revoked)
    }

    async fn redeem_code(
        &self,
        _code: TokenDigest,
        _family: Uuid,
    ) -> anyhow::Result<CodeRedemption> {
        Ok(CodeRedemption::Unknown)
    }

    async fn revoke_refresh_family(&self, _family: Uuid) -> anyhow::Result<()> {
        Ok(())
    }

    async fn rotate_refresh_token(
        &self,
        _presented: TokenDigest,
        _next: TokenDigest,
    ) -> anyhow::Result<RefreshRotation> {
        Ok(RefreshRotation::Unknown)
    }

    async fn take_pending_authorization(
        &self,
        _id: Uuid,
    ) -> anyhow::Result<PendingAuthorizationTake> {
        Ok(PendingAuthorizationTake::Absent)
    }
}
