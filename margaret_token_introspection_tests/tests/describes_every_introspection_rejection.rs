use chrono::DateTime;
use chrono::Utc;

use margaret_token_introspection::introspection_rejection::IntrospectionRejection;

fn instant(seconds: i64) -> DateTime<Utc> {
    DateTime::from_timestamp(seconds, 0).expect("the fixture instant is representable")
}

#[test]
fn describes_every_introspection_rejection() {
    let described = [
        IntrospectionRejection::AudienceMismatch {
            found: vec!["other".to_string()],
        },
        IntrospectionRejection::AudienceMissing,
        IntrospectionRejection::Expired {
            exp: instant(10),
            now: instant(20),
        },
        IntrospectionRejection::Inactive,
        IntrospectionRejection::IssuerMismatch {
            found: "https://attacker.example".to_string(),
        },
        IntrospectionRejection::NotYetValid {
            nbf: instant(30),
            now: instant(20),
        },
        IntrospectionRejection::UnexpectedClaims {
            source: serde_json::from_str::<u8>("x").expect_err("not json"),
        },
    ]
    .map(|rejection| rejection.to_string());

    assert_eq!(
        described[0],
        "the introspected token's audience [\"other\"] does not include ours"
    );
    assert_eq!(described[1], "the introspected token names no audience");
    assert_eq!(
        described[2],
        "the introspected token expired at 1970-01-01 00:00:10 UTC, it is 1970-01-01 00:00:20 UTC"
    );
    assert_eq!(described[3], "the introspected token is not active");
    assert_eq!(
        described[4],
        "the introspected token was issued by 'https://attacker.example' instead of its authorization server"
    );
    assert_eq!(
        described[5],
        "the introspected token is not valid before 1970-01-01 00:00:30 UTC, it is 1970-01-01 00:00:20 UTC"
    );
    assert!(
        described[6].starts_with("the introspected token does not carry the expected claims: ")
    );
}
