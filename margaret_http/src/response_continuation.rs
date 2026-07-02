use bytes::Bytes;
use http_body_util::Full;

use crate::deferred_interception::DeferredInterception;
use crate::forward::Forward;
use crate::response::Response;

pub enum Responded {
    Done(Response),
    Forward(Forward),
    Intercept(Box<dyn DeferredInterception>),
}

impl Responded {
    pub(crate) fn into_http(self) -> http::Response<Full<Bytes>> {
        match self {
            Self::Done(response) => response.into_http(),
            Self::Forward(_) | Self::Intercept(_) => {
                Response::text(500, "Internal Server Error").into_http()
            }
        }
    }
}

impl From<Response> for Responded {
    fn from(response: Response) -> Self {
        Self::Done(response)
    }
}

impl From<Forward> for Responded {
    fn from(forward: Forward) -> Self {
        Self::Forward(forward)
    }
}

#[cfg(test)]
mod tests {
    use super::Responded;
    use crate::forward::Forward;
    use crate::response::Response;

    #[test]
    fn renders_a_done_outcome_as_its_response() {
        assert_eq!(
            Responded::from(Response::text(201, "created"))
                .into_http()
                .status()
                .as_u16(),
            201
        );
    }

    #[test]
    fn renders_an_unresolved_outcome_as_a_server_error() {
        assert_eq!(
            Responded::from(Forward::to("crate::Target"))
                .into_http()
                .status()
                .as_u16(),
            500
        );
    }
}
