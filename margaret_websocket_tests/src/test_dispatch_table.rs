use std::collections::HashMap;
use std::sync::Arc;

use margaret_websocket::web_socket_dispatch_table::WebSocketDispatchTable;
use margaret_websocket::web_socket_message_dispatch::WebSocketMessageDispatch;
use margaret_websocket::web_socket_notification_dispatch::WebSocketNotificationDispatch;

use crate::cleanup_dispatch::CleanupDispatch;
use crate::cleanup_handler::CleanupHandler;
use crate::failing_dispatch::FailingDispatch;
use crate::failing_handler::FailingHandler;
use crate::flood_dispatch::FloodDispatch;
use crate::flood_handler::FloodHandler;
use crate::ping_dispatch::PingDispatch;
use crate::ping_handler::PingHandler;
use crate::storyboard_dispatch::StoryboardDispatch;
use crate::storyboard_handler::StoryboardHandler;
use crate::test_session::TestSession;
use crate::typing_dispatch::TypingDispatch;
use crate::typing_handler::TypingHandler;

#[must_use]
pub fn test_dispatch_table() -> Arc<WebSocketDispatchTable<TestSession>> {
    let mut requests: HashMap<String, Arc<dyn WebSocketMessageDispatch<TestSession>>> =
        HashMap::new();

    requests.insert(
        "conversation_message".to_string(),
        Arc::new(StoryboardDispatch {
            handler: Arc::new(StoryboardHandler),
        }),
    );
    requests.insert(
        "ping".to_string(),
        Arc::new(PingDispatch {
            handler: Arc::new(PingHandler),
        }),
    );
    requests.insert(
        "failing".to_string(),
        Arc::new(FailingDispatch {
            handler: Arc::new(FailingHandler),
        }),
    );
    requests.insert(
        "flood".to_string(),
        Arc::new(FloodDispatch {
            handler: Arc::new(FloodHandler),
        }),
    );
    requests.insert(
        "cleanup".to_string(),
        Arc::new(CleanupDispatch {
            handler: Arc::new(CleanupHandler),
        }),
    );

    let mut notifications: HashMap<String, Arc<dyn WebSocketNotificationDispatch<TestSession>>> =
        HashMap::new();

    notifications.insert(
        "typing".to_string(),
        Arc::new(TypingDispatch {
            handler: Arc::new(TypingHandler),
        }),
    );

    Arc::new(WebSocketDispatchTable::new(requests, notifications))
}
