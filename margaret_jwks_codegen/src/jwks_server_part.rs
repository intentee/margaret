#[derive(Clone, Copy, Eq, Ord, PartialEq, PartialOrd)]
pub enum JwksServerPart {
    Handler,
    Minter,
    Roller,
    SecretStore,
}
