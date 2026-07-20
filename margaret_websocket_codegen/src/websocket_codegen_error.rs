use thiserror::Error;

use margaret_attributes::attribute_error::AttributeError;
use margaret_injection_codegen::injection_error::InjectionError;

#[derive(Debug, Error)]
pub enum WebsocketCodegenError {
    #[error("failed to index the crate: {source}")]
    Index {
        #[from]
        source: AttributeError,
    },

    #[error("failed to resolve a transition's #[process] method: {source}")]
    Injection {
        #[from]
        source: InjectionError,
    },

    #[error("#[websocket_state] is only supported on structs, but '{state}' is not a struct")]
    StateNotAStruct { state: String },

    #[error("#[websocket_state] '{state}' is declared more than once")]
    DuplicateState { state: String },

    #[error(
        "entry state '{state}' declares 'server' but is missing the matching 'path' argument"
    )]
    EntryMissingPath { state: String },

    #[error(
        "entry state '{state}' declares 'path' but is missing the matching 'server' argument"
    )]
    EntryMissingServer { state: String },

    #[error(
        "state '{state}' declares an entry route and 'terminal' at once; a state is either an entry, an intermediate, or a terminal"
    )]
    ConflictingStateRole { state: String },

    #[error("#[websocket_message] is only supported on structs, but '{message}' is not a struct")]
    MessageNotAStruct { message: String },

    #[error("#[websocket_message] '{message}' is declared more than once")]
    DuplicateMessage { message: String },

    #[error("#[websocket_message] '{message}' is missing the required 'method' argument")]
    MessageMissingMethod { message: String },

    #[error("#[websocket_message] '{message}' declares an empty wire method")]
    EmptyWireMethod { message: String },

    #[error(
        "messages '{first}' and '{second}' both declare the wire method '{method}'; a wire method identifies exactly one message"
    )]
    DuplicateWireMethod {
        method: String,
        first: String,
        second: String,
    },

    #[error(
        "#[websocket_internal_event] is only supported on structs, but '{event}' is not a struct"
    )]
    InternalEventNotAStruct { event: String },

    #[error("#[websocket_internal_event] '{event}' is declared more than once")]
    DuplicateInternalEvent { event: String },

    #[error(
        "#[websocket_transition] is only supported on structs, but '{transition}' is not a struct"
    )]
    TransitionNotAStruct { transition: String },

    #[error("#[websocket_transition] '{transition}' must also carry #[singleton]")]
    TransitionNotSingleton { transition: String },

    #[error("#[websocket_transition] '{transition}' is missing the required 'from' argument")]
    MissingTransitionFrom { transition: String },

    #[error("#[websocket_transition] '{transition}' is missing the required 'on' argument")]
    MissingTransitionOn { transition: String },

    #[error(
        "#[websocket_transition] '{transition}' declares from = '{written}', which is not a #[websocket_state]"
    )]
    FromNotAState { transition: String, written: String },

    #[error(
        "#[websocket_transition] '{transition}' declares on = '{written}', which is neither a #[websocket_message] nor a #[websocket_internal_event]"
    )]
    OnNotAMessageOrEvent { transition: String, written: String },

    #[error(
        "#[websocket_transition] '{transition}' declares emits('{written}'), which is not a #[websocket_message]"
    )]
    EmitsNotAMessage { transition: String, written: String },

    #[error(
        "#[websocket_transition] '{transition}' must return its next #[websocket_state] by value"
    )]
    TransitionMissingReturnState { transition: String },

    #[error(
        "#[websocket_transition] '{transition}' returns '{written}', which is not a #[websocket_state]"
    )]
    NextNotAState { transition: String, written: String },

    #[error(
        "#[websocket_transition] '{transition}' has a #[process] parameter '{written}' that is not an injectable transition input (from-state, Envelope, internal event, &Emit, &ConnectionFacts, or &ActivitySpawner)"
    )]
    UnclassifiableTransitionParameter { transition: String, written: String },

    #[error(
        "transitions '{first}' and '{second}' both dispatch from state '{from}' on '{on}'; a (state, trigger) pair maps to exactly one transition"
    )]
    DuplicateTransition {
        from: String,
        on: String,
        first: String,
        second: String,
    },

    #[error(
        "no #[websocket_state] declares an entry route; every protocol needs a state carrying #[websocket_state(server = ..., path = ...)]"
    )]
    MissingEntryState,

    #[error(
        "entry states '{first}' and '{second}' both declare the route ('{server}', '{path}'); each route roots exactly one protocol"
    )]
    DuplicateProtocolRoute {
        server: String,
        path: String,
        first: String,
        second: String,
    },

    #[error(
        "terminal state '{state}' has an outgoing transition on '{on}'; a terminal state must not transition further"
    )]
    TerminalStateHasTransitions { state: String, on: String },

    #[error(
        "state '{state}' is reachable from both entry states '{first}' and '{second}'; a state belongs to exactly one protocol"
    )]
    AmbiguousProtocolMembership {
        state: String,
        first: String,
        second: String,
    },

    #[error(
        "state '{state}' is not reachable from any entry state; every declared state must be part of a protocol"
    )]
    UnreachableState { state: String },

    #[error(
        "state '{state}' is a dead end; no terminal state is reachable from it, so the protocol can never close cleanly"
    )]
    DeadEndState { state: String },
}
