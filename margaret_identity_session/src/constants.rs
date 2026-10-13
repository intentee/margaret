pub const ACCESS_TOKEN_LIFETIME_SECS: u32 = 15 * 60;

pub const ID_TOKEN_LIFETIME_SECS: u32 = 15 * 60;

pub const ID_TOKEN_MEMBERS: [&str; 8] = [
    "aud",
    "auth_time",
    "azp",
    "exp",
    "iat",
    "iss",
    "nonce",
    "sub",
];

pub const SIGN_IN_TRANSACTION_LIFETIME_SECS: u32 = 10 * 60;
