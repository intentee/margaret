use tokio::sync::mpsc::UnboundedSender;
use tokio::sync::mpsc::error::SendError;

use crate::connection_driver::ConnectionDriver;

#[derive(Clone)]
pub struct ConnectionDriverSender {
    inner: UnboundedSender<ConnectionDriver>,
}

impl ConnectionDriverSender {
    pub(crate) fn new(inner: UnboundedSender<ConnectionDriver>) -> Self {
        Self { inner }
    }

    pub fn send(&self, driver: ConnectionDriver) -> Result<(), ConnectionDriver> {
        self.inner
            .send(driver)
            .map_err(|SendError(rejected)| rejected)
    }
}
