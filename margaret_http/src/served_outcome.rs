use crate::response::Response;
use crate::upgrade_handler::UpgradeHandler;

pub enum ServedOutcome {
    Http(Response),
    Upgrade(Box<dyn UpgradeHandler>),
}

#[cfg(test)]
impl ServedOutcome {
    pub(crate) fn expect_http(self) -> Response {
        match self {
            ServedOutcome::Http(response) => response,
            ServedOutcome::Upgrade(handler) => handler.switching_response(),
        }
    }
}
