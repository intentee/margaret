use futures_util::future::join_all;
use uuid::Uuid;

use margaret_authorization_grants::family_opening::FamilyOpening;
use margaret_authorization_grants::refresh_family_record::RefreshFamilyRecord;
use margaret_authorization_grants::refresh_rotation::RefreshRotation;
use margaret_authorization_grants::refresh_token_lookup::RefreshTokenLookup;
use margaret_authorization_grants::refresh_token_record::RefreshTokenRecord;
use margaret_authorization_grants_tests::contract_refresh_family::contract_refresh_family;
use margaret_authorization_grants_tests::started_with_authorization_grants::started_with_authorization_grants;
use margaret_database_tests::contract_instant::contract_instant;
use margaret_database_tests::contract_token::contract_token;
use margaret_database_tests::racing_instances::RACING_INSTANCES;
use margaret_token_digest::token_digest::TokenDigest;

struct RacedRotation {
    next: TokenDigest,
    rotation: RefreshRotation,
}

#[tokio::test]
async fn concurrent_refresh_rotations_admit_one() {
    let started = started_with_authorization_grants().await;
    let database = started.database.as_ref();
    let family = Uuid::new_v4();
    let record = contract_refresh_family();
    let presented = contract_token();
    let now = contract_instant();

    assert_eq!(
        RefreshFamilyRecord::open(database, family, record.clone(), presented, now)
            .await
            .expect("the database opens the refresh family"),
        FamilyOpening::Opened
    );

    let raced: Vec<RacedRotation> = join_all((0..RACING_INSTANCES).map(|_| async move {
        let next = contract_token();

        RacedRotation {
            next,
            rotation: RefreshTokenRecord::rotate(database, presented, next, now)
                .await
                .expect("the database rotates the refresh token"),
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
        RefreshTokenRecord::lookup(database, rotated[0])
            .await
            .expect("the database finds refresh tokens"),
        RefreshTokenLookup::Current { family, record }
    );
    assert_eq!(
        RefreshTokenRecord::lookup(database, presented)
            .await
            .expect("the database finds refresh tokens"),
        RefreshTokenLookup::Superseded { family }
    );

    for raced in raced
        .iter()
        .filter(|raced| raced.rotation != RefreshRotation::Rotated)
    {
        assert_eq!(
            RefreshTokenRecord::lookup(database, raced.next)
                .await
                .expect("the database finds refresh tokens"),
            RefreshTokenLookup::Unknown
        );
    }
}
