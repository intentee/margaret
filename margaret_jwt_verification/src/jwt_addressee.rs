use margaret_registered_claims::issuer_identifier::IssuerIdentifier;

pub trait JwtAddressee {
    fn jwt_issuer(&self) -> &IssuerIdentifier;
}
