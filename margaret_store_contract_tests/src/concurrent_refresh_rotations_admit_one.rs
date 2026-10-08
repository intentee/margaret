use futures_util::future::join_all;
use uuid::Uuid;

use margaret_authorization_grants::family_opening::FamilyOpening;
use margaret_authorization_grants::refresh_rotation::RefreshRotation;
use margaret_authorization_grants::refresh_token_lookup::RefreshTokenLookup;
use margaret_authorization_grants::stores_authorization_grants::StoresAuthorizationGrants;
use margaret_token_digest::token_digest::TokenDigest;

use crate::contract_refresh_family::contract_refresh_family;
use crate::contract_token::contract_token;
use crate::racing_instances::RACING_INSTANCES;

struct RacedRotation {
    next: TokenDigest,
    rotation: RefreshRotation,
}

/// # Panics
///
/// Panics when the store breaks this clause of its contract or cannot be reached.
pub async fn concurrent_refresh_rotations_admit_one(store: &dyn StoresAuthorizationGrants) {
    let family = Uuid::new_v4();
    let record = contract_refresh_family();
    let presented = contract_token();

    assert_eq!(
        store
            .open_refresh_family(family, record.clone(), presented)
            .await
            .expect("the store opens the refresh family"),
        FamilyOpening::Opened
    );

    let raced: Vec<RacedRotation> = join_all((0..RACING_INSTANCES).map(|_| async move {
        let next = contract_token();

        RacedRotation {
            next,
            rotation: store
                .rotate_refresh_token(presented, next)
                .await
                .expect("the store rotates the refresh token"),
        }
    }))
    .await;
    let rotated: Vec<TokenDigest> = raced
        .iter()
        .filter(|raced| raced.rotation == RefreshRotation::Rotated)
        .map(|raced| raced.next)
        .collect();

    assert_eq!(rotated.len(), 1);
    assert_eq!(
        raced
            .iter()
            .filter(|raced| raced.rotation == RefreshRotation::Superseded { family })
            .count(),
        RACING_INSTANCES - 1
    );
    assert_eq!(
        store
            .find_refresh_token(rotated[0])
            .await
            .expect("the store finds refresh tokens"),
        RefreshTokenLookup::Current { family, record }
    );
    assert_eq!(
        store
            .find_refresh_token(presented)
            .await
            .expect("the store finds refresh tokens"),
        RefreshTokenLookup::Superseded { family }
    );

    for raced in raced
        .iter()
        .filter(|raced| raced.rotation != RefreshRotation::Rotated)
    {
        assert_eq!(
            store
                .find_refresh_token(raced.next)
                .await
                .expect("the store finds refresh tokens"),
            RefreshTokenLookup::Unknown
        );
    }
}
