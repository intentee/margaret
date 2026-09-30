use std::iter;

use headers::Authorization;
use headers::Header;
use headers::authorization::Basic;
use http::HeaderValue;
use zeroize::Zeroizing;

use crate::form_decoded::form_decoded;

fn basic_credentials(
    authorization: &str,
    form_client_id: Option<&str>,
) -> PresentedClientCredentials {
    let Ok(value) = HeaderValue::from_str(authorization) else {
        return PresentedClientCredentials::Malformed;
    };
    let Ok(basic) = Authorization::<Basic>::decode(&mut iter::once(&value)) else {
        return PresentedClientCredentials::Malformed;
    };
    let Ok(client_id) = form_decoded(basic.username()) else {
        return PresentedClientCredentials::Malformed;
    };
    let Ok(secret) = form_decoded(basic.password()) else {
        return PresentedClientCredentials::Malformed;
    };

    match form_client_id {
        Some(form_client_id) if form_client_id != client_id => {
            PresentedClientCredentials::ConflictingClientIds
        }
        _ => PresentedClientCredentials::Basic {
            client_id,
            secret: Zeroizing::new(secret),
        },
    }
}

pub enum PresentedClientCredentials {
    Absent,
    Basic {
        client_id: String,
        secret: Zeroizing<String>,
    },
    ClientId(String),
    ConflictingClientIds,
    Malformed,
}

impl PresentedClientCredentials {
    #[must_use]
    pub fn of(authorization: Option<&str>, form_client_id: Option<&str>) -> Self {
        match authorization {
            Some(authorization) => basic_credentials(authorization, form_client_id),
            None => match form_client_id {
                Some(client_id) => Self::ClientId(client_id.to_string()),
                None => Self::Absent,
            },
        }
    }
}
