use std::collections::HashMap;
use std::sync::Arc;

use crate::web_socket_message_dispatch::WebSocketMessageDispatch;
use crate::web_socket_notification_dispatch::WebSocketNotificationDispatch;

pub struct WebSocketDispatchTable<Session> {
    notifications: HashMap<String, Arc<dyn WebSocketNotificationDispatch<Session>>>,
    requests: HashMap<String, Arc<dyn WebSocketMessageDispatch<Session>>>,
}

impl<Session> WebSocketDispatchTable<Session> {
    #[must_use]
    pub fn new(
        requests: HashMap<String, Arc<dyn WebSocketMessageDispatch<Session>>>,
        notifications: HashMap<String, Arc<dyn WebSocketNotificationDispatch<Session>>>,
    ) -> Self {
        Self {
            notifications,
            requests,
        }
    }

    pub(crate) fn notification(
        &self,
        method: &str,
    ) -> Option<&Arc<dyn WebSocketNotificationDispatch<Session>>> {
        self.notifications.get(method)
    }

    pub(crate) fn request(
        &self,
        method: &str,
    ) -> Option<&Arc<dyn WebSocketMessageDispatch<Session>>> {
        self.requests.get(method)
    }
}
