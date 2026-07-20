use margaret_attributes::canonical_path::CanonicalPath;

use crate::emitted_message::EmittedMessage;
use crate::transition_trigger::TransitionTrigger;
use crate::websocket_injectable::WebsocketInjectable;

#[derive(Debug)]
pub struct WebsocketTransition {
    pub transition: CanonicalPath,
    pub field: String,
    pub type_name: String,
    pub process_method: String,
    pub from: CanonicalPath,
    pub trigger: TransitionTrigger,
    pub emits: Vec<EmittedMessage>,
    pub next: CanonicalPath,
    pub bindings: Vec<WebsocketInjectable>,
}
