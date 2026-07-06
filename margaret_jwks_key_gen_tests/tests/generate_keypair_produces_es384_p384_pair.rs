use anyhow::Result;
use margaret_jwks_key_gen::curve::Curve;
use margaret_jwks_key_gen::generate_keypair::generate_keypair;
use margaret_jwks_key_gen::generate_keypair_params::GenerateKeypairParams;
use margaret_jwks_key_gen::key_type::KeyType;
use margaret_jwks_key_gen::key_use::KeyUse;

#[test]
fn generate_keypair_produces_es384_p384_pair() -> Result<()> {
    let keypair = generate_keypair(GenerateKeypairParams {
        crv: Curve::P384,
        kid: "generated-kid".to_string(),
    })?;

    assert!(matches!(keypair.public.crv, Curve::P384));
    assert!(matches!(keypair.public.kty, KeyType::Ec));
    assert!(matches!(keypair.public.use_, KeyUse::Signature));
    assert_eq!(keypair.public.kid, "generated-kid");
    assert_eq!(keypair.signing.kid, "generated-kid");
    assert!(!keypair.public.x.is_empty());
    assert!(!keypair.public.y.is_empty());

    Ok(())
}
