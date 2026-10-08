use std::collections::HashMap;

use uuid::Uuid;

use margaret_authorization_grants::pending_authorization::PendingAuthorization;
use margaret_authorization_grants::refresh_rotation::RefreshRotation;
use margaret_token_digest::random_token::random_token;
use margaret_token_digest::token_digest::TokenDigest;

use crate::fixture_code::FixtureCode;
use crate::fixture_family::FixtureFamily;
use crate::fixture_refresh_token::FixtureRefreshToken;
use crate::grant_interference::GrantInterference;

#[derive(Default)]
pub(crate) struct FixtureGrantRecords {
    pub(crate) codes: HashMap<TokenDigest, FixtureCode>,
    pub(crate) families: HashMap<Uuid, FixtureFamily>,
    pub(crate) pending: HashMap<Uuid, PendingAuthorization>,
    pub(crate) tokens: HashMap<TokenDigest, FixtureRefreshToken>,
}

impl FixtureGrantRecords {
    pub(crate) fn open_family_of(&self, token: &TokenDigest) -> Option<&FixtureRefreshToken> {
        self.tokens.get(token).filter(|held| {
            matches!(
                self.families.get(&held.family),
                Some(FixtureFamily::Open(_))
            )
        })
    }

    pub(crate) fn rotate_current(
        &mut self,
        interference: GrantInterference,
        presented: TokenDigest,
        next: TokenDigest,
        family: Uuid,
    ) -> RefreshRotation {
        match interference {
            GrantInterference::RotationBeforeRefreshRotates => {
                self.supersede(presented, TokenDigest::of(&random_token()), family);

                RefreshRotation::Superseded { family }
            }
            GrantInterference::Failing(_)
            | GrantInterference::ReplayBeforeFamilyOpens
            | GrantInterference::RevocationBeforeRefreshRotates
            | GrantInterference::Undisturbed => {
                self.supersede(presented, next, family);

                RefreshRotation::Rotated
            }
        }
    }

    fn supersede(&mut self, presented: TokenDigest, next: TokenDigest, family: Uuid) {
        self.tokens.insert(
            presented,
            FixtureRefreshToken {
                current: false,
                family,
            },
        );
        self.tokens.insert(
            next,
            FixtureRefreshToken {
                current: true,
                family,
            },
        );
    }
}
