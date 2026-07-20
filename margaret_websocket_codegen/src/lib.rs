pub mod has_websocket;
pub mod render_websocket;
pub mod websocket_artifacts;
pub mod websocket_codegen_error;

mod alphabet_ident;
mod emitted_message;
mod protocol;
mod protocols;
mod render_binding_arguments;
mod render_dispatch;
mod render_dispatch_internal;
mod render_dispatcher;
mod render_handshake_handler;
mod render_internal;
mod render_message_outbound;
mod render_protocol;
mod render_protocol_state;
mod render_transition_alphabet;
mod state_role;
mod transition_trigger;
mod websocket_injectable;
mod websocket_internal_event;
mod websocket_internal_events;
mod websocket_message;
mod websocket_messages;
mod websocket_state;
mod websocket_states;
mod websocket_transition;
mod websocket_transitions;

#[cfg(test)]
mod tests {
    use std::fs;

    use tempfile::TempDir;
    use tempfile::tempdir;

    use margaret_attributes::attribute_index::AttributeIndex;
    use margaret_attributes::attribute_index_builder::AttributeIndexBuilder;
    use margaret_attributes::crate_root::CrateRoot;

    use crate::has_websocket::has_websocket;
    use crate::protocols::protocols;
    use crate::render_websocket::render_websocket;
    use crate::state_role::StateRole;
    use crate::transition_trigger::TransitionTrigger;
    use crate::websocket_artifacts::WebsocketArtifacts;
    use crate::websocket_codegen_error::WebsocketCodegenError;
    use crate::websocket_injectable::WebsocketInjectable;
    use crate::websocket_internal_events::websocket_internal_events;
    use crate::websocket_messages::websocket_messages;
    use crate::websocket_states::websocket_states;
    use crate::websocket_transition::WebsocketTransition;
    use crate::websocket_transitions::websocket_transitions;

    fn crate_with(lib_source: &str) -> TempDir {
        let directory = tempdir().expect("a temporary crate directory is created");
        let source_directory = directory.path().join("src");

        fs::create_dir(&source_directory).expect("the src directory is created");
        fs::write(source_directory.join("lib.rs"), lib_source).expect("lib.rs is written");

        directory
    }

    fn index_for(lib_source: &str) -> AttributeIndex {
        let directory = crate_with(lib_source);

        AttributeIndexBuilder::new()
            .index_crate(&CrateRoot::new("crate", directory.path().join("src")))
            .expect("the crate is indexed")
            .build()
    }

    fn error_summary(error: &WebsocketCodegenError) -> String {
        match error {
            WebsocketCodegenError::Index { .. } => "index".to_owned(),
            WebsocketCodegenError::Injection { .. } => "injection".to_owned(),
            WebsocketCodegenError::StateNotAStruct { state } => format!("state-not-a-struct {state}"),
            WebsocketCodegenError::DuplicateState { state } => format!("duplicate-state {state}"),
            WebsocketCodegenError::EntryMissingPath { state } => format!("entry-missing-path {state}"),
            WebsocketCodegenError::EntryMissingServer { state } => {
                format!("entry-missing-server {state}")
            }
            WebsocketCodegenError::ConflictingStateRole { state } => {
                format!("conflicting-state-role {state}")
            }
            WebsocketCodegenError::MessageNotAStruct { message } => {
                format!("message-not-a-struct {message}")
            }
            WebsocketCodegenError::DuplicateMessage { message } => {
                format!("duplicate-message {message}")
            }
            WebsocketCodegenError::MessageMissingMethod { message } => {
                format!("message-missing-method {message}")
            }
            WebsocketCodegenError::EmptyWireMethod { message } => {
                format!("empty-wire-method {message}")
            }
            WebsocketCodegenError::DuplicateWireMethod { method, first, second } => {
                format!("duplicate-wire-method {method} {first} {second}")
            }
            WebsocketCodegenError::InternalEventNotAStruct { event } => {
                format!("internal-event-not-a-struct {event}")
            }
            WebsocketCodegenError::DuplicateInternalEvent { event } => {
                format!("duplicate-internal-event {event}")
            }
            WebsocketCodegenError::TransitionNotAStruct { transition } => {
                format!("transition-not-a-struct {transition}")
            }
            WebsocketCodegenError::TransitionNotSingleton { transition } => {
                format!("transition-not-singleton {transition}")
            }
            WebsocketCodegenError::MissingTransitionFrom { transition } => {
                format!("missing-transition-from {transition}")
            }
            WebsocketCodegenError::MissingTransitionOn { transition } => {
                format!("missing-transition-on {transition}")
            }
            WebsocketCodegenError::FromNotAState { transition, written } => {
                format!("from-not-a-state {transition} {written}")
            }
            WebsocketCodegenError::OnNotAMessageOrEvent { transition, written } => {
                format!("on-not-a-message-or-event {transition} {written}")
            }
            WebsocketCodegenError::EmitsNotAMessage { transition, written } => {
                format!("emits-not-a-message {transition} {written}")
            }
            WebsocketCodegenError::TransitionMissingReturnState { transition } => {
                format!("transition-missing-return-state {transition}")
            }
            WebsocketCodegenError::NextNotAState { transition, written } => {
                format!("next-not-a-state {transition} {written}")
            }
            WebsocketCodegenError::UnclassifiableTransitionParameter { transition, written } => {
                format!("unclassifiable-transition-parameter {transition} {written}")
            }
            WebsocketCodegenError::DuplicateTransition { from, on, .. } => {
                format!("duplicate-transition {from} {on}")
            }
            WebsocketCodegenError::MissingEntryState => "missing-entry-state".to_owned(),
            WebsocketCodegenError::DuplicateProtocolRoute { server, path, .. } => {
                format!("duplicate-protocol-route {server} {path}")
            }
            WebsocketCodegenError::TerminalStateHasTransitions { state, on } => {
                format!("terminal-state-has-transitions {state} {on}")
            }
            WebsocketCodegenError::AmbiguousProtocolMembership { state, .. } => {
                format!("ambiguous-protocol-membership {state}")
            }
            WebsocketCodegenError::UnreachableState { state } => {
                format!("unreachable-state {state}")
            }
            WebsocketCodegenError::DeadEndState { state } => format!("dead-end-state {state}"),
        }
    }

    const STORYBOARD: &str = "\
#[websocket_state(server = \"public\", path = \"/storyboard\")]
struct Fresh;

#[websocket_state]
struct Thinking;

#[websocket_state(terminal)]
struct Ended;

#[websocket_transition(from = crate::Fresh, on = crate::Speak)]
struct BeginChat;
";

    #[test]
    fn detects_the_presence_of_websocket_protocols() {
        assert!(has_websocket(&index_for(STORYBOARD)));
        assert!(!has_websocket(&index_for("struct Plain;\n")));
    }

    fn role_label(role: &StateRole) -> String {
        match role {
            StateRole::Entry { server, path } => format!("entry({server},{path})"),
            StateRole::Intermediate => "intermediate".to_owned(),
            StateRole::Terminal => "terminal".to_owned(),
        }
    }

    #[test]
    fn scans_states_and_records_their_roles() {
        let states = websocket_states(&index_for(STORYBOARD)).expect("the states are scanned");

        let mut role_by_state: Vec<(String, String)> = states
            .into_iter()
            .map(|state| (state.canonical_path.to_string(), role_label(&state.role)))
            .collect();

        role_by_state.sort();

        assert_eq!(
            role_by_state,
            vec![
                ("crate::Ended".to_owned(), "terminal".to_owned()),
                (
                    "crate::Fresh".to_owned(),
                    "entry(public,/storyboard)".to_owned()
                ),
                ("crate::Thinking".to_owned(), "intermediate".to_owned()),
            ]
        );
    }

    #[test]
    fn rejects_an_entry_state_missing_its_path() {
        let error = websocket_states(&index_for(
            "#[websocket_state(server = \"public\")]\nstruct Fresh;\n",
        ))
        .expect_err("an entry state without a path is rejected");

        assert_eq!(error_summary(&error), "entry-missing-path crate::Fresh");
    }

    #[test]
    fn rejects_an_entry_state_missing_its_server() {
        let error = websocket_states(&index_for(
            "#[websocket_state(path = \"/x\")]\nstruct Fresh;\n",
        ))
        .expect_err("an entry state without a server is rejected");

        assert_eq!(error_summary(&error), "entry-missing-server crate::Fresh");
    }

    #[test]
    fn rejects_a_state_that_is_both_an_entry_and_terminal() {
        let error = websocket_states(&index_for(
            "#[websocket_state(server = \"public\", path = \"/x\", terminal)]\nstruct Fresh;\n",
        ))
        .expect_err("a state cannot be both an entry and terminal");

        assert_eq!(error_summary(&error), "conflicting-state-role crate::Fresh");
    }

    #[test]
    fn rejects_a_state_that_is_not_a_struct() {
        let error = websocket_states(&index_for("#[websocket_state]\nenum NotAStruct {}\n"))
            .expect_err("a non-struct state is rejected");

        assert_eq!(error_summary(&error), "state-not-a-struct crate::NotAStruct");
    }

    #[test]
    fn rejects_a_state_declared_more_than_once() {
        let error = websocket_states(&index_for(
            "#[websocket_state]\n#[websocket_state]\nstruct Twice;\n",
        ))
        .expect_err("a doubly-declared state is rejected");

        assert_eq!(error_summary(&error), "duplicate-state crate::Twice");
    }

    #[test]
    fn scans_messages_with_their_wire_methods() {
        let messages = websocket_messages(&index_for(
            "#[websocket_message(method = \"conversation.message\")]\nstruct Speak;\n",
        ))
        .expect("the messages are scanned");

        let methods: Vec<(String, String)> = messages
            .into_iter()
            .map(|message| (message.canonical_path.to_string(), message.method))
            .collect();

        assert_eq!(
            methods,
            vec![(
                "crate::Speak".to_owned(),
                "conversation.message".to_owned()
            )]
        );
    }

    #[test]
    fn rejects_a_message_that_is_not_a_struct() {
        let error = websocket_messages(&index_for(
            "#[websocket_message(method = \"m\")]\nenum NotAStruct {}\n",
        ))
        .expect_err("a non-struct message is rejected");

        assert_eq!(error_summary(&error), "message-not-a-struct crate::NotAStruct");
    }

    #[test]
    fn rejects_a_message_declared_more_than_once() {
        let error = websocket_messages(&index_for(
            "#[websocket_message(method = \"a\")]\n#[websocket_message(method = \"b\")]\nstruct Twice;\n",
        ))
        .expect_err("a doubly-declared message is rejected");

        assert_eq!(error_summary(&error), "duplicate-message crate::Twice");
    }

    #[test]
    fn rejects_a_message_without_a_method() {
        let error = websocket_messages(&index_for("#[websocket_message]\nstruct NoMethod;\n"))
            .expect_err("a message without a method is rejected");

        assert_eq!(error_summary(&error), "message-missing-method crate::NoMethod");
    }

    #[test]
    fn rejects_an_empty_wire_method() {
        let error = websocket_messages(&index_for(
            "#[websocket_message(method = \"\")]\nstruct Empty;\n",
        ))
        .expect_err("an empty wire method is rejected");

        assert_eq!(error_summary(&error), "empty-wire-method crate::Empty");
    }

    #[test]
    fn rejects_two_messages_sharing_a_wire_method() {
        let error = websocket_messages(&index_for(
            "#[websocket_message(method = \"shared\")]\nstruct First;\n\n#[websocket_message(method = \"shared\")]\nstruct Second;\n",
        ))
        .expect_err("two messages sharing a wire method are rejected");

        assert_eq!(error_summary(&error), "duplicate-wire-method shared crate::First crate::Second");
    }

    #[test]
    fn propagates_an_attribute_argument_error() {
        let error = websocket_messages(&index_for(
            "#[websocket_message(method = 5)]\nstruct NumericMethod;\n",
        ))
        .expect_err("a non-string method surfaces as an indexing error");

        assert_eq!(error_summary(&error), "index");
    }

    #[test]
    fn scans_internal_events() {
        let events = websocket_internal_events(&index_for(
            "#[websocket_internal_event]\nstruct GenerationComplete;\n",
        ))
        .expect("the internal events are scanned");

        let paths: Vec<String> = events
            .into_iter()
            .map(|event| event.canonical_path.to_string())
            .collect();

        assert_eq!(paths, vec!["crate::GenerationComplete".to_owned()]);
    }

    #[test]
    fn rejects_an_internal_event_that_is_not_a_struct() {
        let error = websocket_internal_events(&index_for(
            "#[websocket_internal_event]\nenum NotAStruct {}\n",
        ))
        .expect_err("a non-struct internal event is rejected");

        assert_eq!(error_summary(&error), "internal-event-not-a-struct crate::NotAStruct");
    }

    #[test]
    fn rejects_an_internal_event_declared_more_than_once() {
        let error = websocket_internal_events(&index_for(
            "#[websocket_internal_event]\n#[websocket_internal_event]\nstruct Twice;\n",
        ))
        .expect_err("a doubly-declared internal event is rejected");

        assert_eq!(error_summary(&error), "duplicate-internal-event crate::Twice");
    }

    fn scan_transitions(
        lib_source: &str,
    ) -> Result<Vec<WebsocketTransition>, WebsocketCodegenError> {
        let index = index_for(lib_source);
        let states = websocket_states(&index)?;
        let messages = websocket_messages(&index)?;
        let events = websocket_internal_events(&index)?;

        websocket_transitions(&index, &states, &messages, &events)
    }

    const STORYBOARD_STATES: &str = "\
#[websocket_state(server = \"public\", path = \"/storyboard\")]
struct Fresh;

#[websocket_state]
struct Thinking;

#[websocket_message(method = \"conversation.speak\")]
struct Speak;

#[websocket_message(method = \"assistant.accepted\")]
struct Accepted;

#[websocket_internal_event]
struct GenerationComplete;
";

    #[test]
    fn scans_a_wire_message_transition_with_its_graph_edges() {
        let source = format!(
            "{STORYBOARD_STATES}\n#[singleton]\n#[websocket_transition(from = crate::Fresh, on = crate::Speak, emits(crate::Accepted))]\nstruct BeginChat;\n\nimpl BeginChat {{\n    #[process]\n    fn on(&self) -> crate::Thinking {{}}\n}}\n"
        );

        let transitions = scan_transitions(&source).expect("the transition is scanned");

        assert_eq!(transitions.len(), 1);

        let transition = &transitions[0];

        assert_eq!(transition.transition.to_string(), "crate::BeginChat");
        assert_eq!(transition.process_method, "on");
        assert_eq!(transition.from.to_string(), "crate::Fresh");
        assert_eq!(transition.next.to_string(), "crate::Thinking");
        assert_eq!(
            transition
                .emits
                .iter()
                .map(|emit| emit.canonical_path.to_string())
                .collect::<Vec<String>>(),
            vec!["crate::Accepted".to_owned()]
        );
        assert!(matches!(
            &transition.trigger,
            TransitionTrigger::Message { canonical_path, method, .. }
                if canonical_path.to_string() == "crate::Speak" && method == "conversation.speak"
        ));
    }

    #[test]
    fn scans_an_internal_event_transition() {
        let source = format!(
            "{STORYBOARD_STATES}\n#[singleton]\n#[websocket_transition(from = crate::Thinking, on = crate::GenerationComplete)]\nstruct Complete;\n\nimpl Complete {{\n    #[process]\n    fn on(&self) -> crate::Fresh {{}}\n}}\n"
        );

        let transitions = scan_transitions(&source).expect("the transition is scanned");

        assert!(matches!(
            &transitions[0].trigger,
            TransitionTrigger::InternalEvent { canonical_path, .. }
                if canonical_path.to_string() == "crate::GenerationComplete"
        ));
    }

    #[test]
    fn rejects_a_transition_that_is_not_a_struct() {
        let error = scan_transitions(
            "#[websocket_transition(from = crate::A, on = crate::B)]\nenum NotAStruct {}\n",
        )
        .expect_err("a non-struct transition is rejected");

        assert_eq!(error_summary(&error), "transition-not-a-struct crate::NotAStruct");
    }

    #[test]
    fn rejects_a_transition_without_singleton() {
        let error = scan_transitions(
            "#[websocket_transition(from = crate::A, on = crate::B)]\nstruct Bare;\n",
        )
        .expect_err("a transition without #[singleton] is rejected");

        assert_eq!(error_summary(&error), "transition-not-singleton crate::Bare");
    }

    #[test]
    fn rejects_a_transition_missing_from() {
        let error = scan_transitions(
            "#[singleton]\n#[websocket_transition(on = crate::B)]\nstruct NoFrom;\n",
        )
        .expect_err("a transition without 'from' is rejected");

        assert_eq!(error_summary(&error), "missing-transition-from crate::NoFrom");
    }

    #[test]
    fn rejects_a_transition_missing_on() {
        let error = scan_transitions(
            "#[singleton]\n#[websocket_transition(from = crate::A)]\nstruct NoOn;\n",
        )
        .expect_err("a transition without 'on' is rejected");

        assert_eq!(error_summary(&error), "missing-transition-on crate::NoOn");
    }

    #[test]
    fn rejects_from_that_is_not_a_state() {
        let source = format!(
            "{STORYBOARD_STATES}\n#[singleton]\n#[websocket_transition(from = crate::Speak, on = crate::Speak)]\nstruct T;\n\nimpl T {{\n    #[process]\n    fn on(&self) -> crate::Thinking {{}}\n}}\n"
        );

        let error = scan_transitions(&source).expect_err("a non-state 'from' is rejected");

        assert_eq!(error_summary(&error), "from-not-a-state crate::T crate :: Speak");
    }

    #[test]
    fn rejects_on_that_is_neither_a_message_nor_an_event() {
        let source = format!(
            "{STORYBOARD_STATES}\n#[singleton]\n#[websocket_transition(from = crate::Fresh, on = crate::Thinking)]\nstruct T;\n\nimpl T {{\n    #[process]\n    fn on(&self) -> crate::Thinking {{}}\n}}\n"
        );

        let error = scan_transitions(&source).expect_err("a non-trigger 'on' is rejected");

        assert_eq!(error_summary(&error), "on-not-a-message-or-event crate::T crate :: Thinking");
    }

    #[test]
    fn rejects_emits_that_is_not_a_message() {
        let source = format!(
            "{STORYBOARD_STATES}\n#[singleton]\n#[websocket_transition(from = crate::Fresh, on = crate::Speak, emits(crate::Thinking))]\nstruct T;\n\nimpl T {{\n    #[process]\n    fn on(&self) -> crate::Thinking {{}}\n}}\n"
        );

        let error = scan_transitions(&source).expect_err("a non-message emit is rejected");

        assert_eq!(error_summary(&error), "emits-not-a-message crate::T crate :: Thinking");
    }

    #[test]
    fn rejects_a_transition_that_returns_no_state() {
        let source = format!(
            "{STORYBOARD_STATES}\n#[singleton]\n#[websocket_transition(from = crate::Fresh, on = crate::Speak)]\nstruct T;\n\nimpl T {{\n    #[process]\n    fn on(&self) {{}}\n}}\n"
        );

        let error = scan_transitions(&source).expect_err("a transition returning nothing is rejected");

        assert_eq!(error_summary(&error), "transition-missing-return-state crate::T");
    }

    #[test]
    fn rejects_a_next_that_is_not_a_state() {
        let source = format!(
            "{STORYBOARD_STATES}\n#[singleton]\n#[websocket_transition(from = crate::Fresh, on = crate::Speak)]\nstruct T;\n\nimpl T {{\n    #[process]\n    fn on(&self) -> crate::Speak {{}}\n}}\n"
        );

        let error = scan_transitions(&source).expect_err("a non-state 'next' is rejected");

        assert_eq!(error_summary(&error), "next-not-a-state crate::T crate :: Speak");
    }

    #[test]
    fn rejects_duplicate_transitions_for_the_same_state_and_trigger() {
        let source = format!(
            "{STORYBOARD_STATES}\n#[singleton]\n#[websocket_transition(from = crate::Fresh, on = crate::Speak)]\nstruct First;\n\nimpl First {{\n    #[process]\n    fn on(&self) -> crate::Thinking {{}}\n}}\n\n#[singleton]\n#[websocket_transition(from = crate::Fresh, on = crate::Speak)]\nstruct Second;\n\nimpl Second {{\n    #[process]\n    fn on(&self) -> crate::Thinking {{}}\n}}\n"
        );

        let error = scan_transitions(&source).expect_err("duplicate transitions are rejected");

        assert_eq!(error_summary(&error), "duplicate-transition crate::Fresh crate::Speak");
    }

    #[test]
    fn classifies_every_injectable_transition_parameter_in_order() {
        let source = format!(
            "{STORYBOARD_STATES}\n#[singleton]\n#[websocket_transition(from = crate::Fresh, on = crate::Speak, emits(crate::Accepted))]\nstruct BeginChat;\n\nimpl BeginChat {{\n    #[process]\n    fn on(\n        &self,\n        state: crate::Fresh,\n        message: margaret_websocket::envelope::Envelope<crate::Speak>,\n        emit: &margaret_websocket::emit::Emit<crate::Alphabet>,\n        facts: &margaret_websocket::connection_facts::ConnectionFacts,\n        spawner: &margaret_websocket::activity_spawner::ActivitySpawner<crate::Internal>,\n    ) -> crate::Thinking {{}}\n}}\n"
        );

        let transitions = scan_transitions(&source).expect("the transition is scanned");

        assert_eq!(
            transitions[0].bindings,
            vec![
                WebsocketInjectable::FromState,
                WebsocketInjectable::Envelope,
                WebsocketInjectable::Emit,
                WebsocketInjectable::Facts,
                WebsocketInjectable::Spawner,
            ]
        );
    }

    #[test]
    fn classifies_an_internal_event_value_parameter() {
        let source = format!(
            "{STORYBOARD_STATES}\n#[singleton]\n#[websocket_transition(from = crate::Thinking, on = crate::GenerationComplete)]\nstruct Complete;\n\nimpl Complete {{\n    #[process]\n    fn on(&self, state: crate::Thinking, event: crate::GenerationComplete) -> crate::Fresh {{}}\n}}\n"
        );

        let transitions = scan_transitions(&source).expect("the transition is scanned");

        assert_eq!(
            transitions[0].bindings,
            vec![
                WebsocketInjectable::FromState,
                WebsocketInjectable::EventValue,
            ]
        );
    }

    #[test]
    fn rejects_an_unclassifiable_transition_parameter() {
        let source = format!(
            "{STORYBOARD_STATES}\n#[singleton]\n#[websocket_transition(from = crate::Fresh, on = crate::Speak)]\nstruct BeginChat;\n\nimpl BeginChat {{\n    #[process]\n    fn on(&self, state: crate::Fresh, junk: crate::Speak) -> crate::Thinking {{}}\n}}\n"
        );

        let error = scan_transitions(&source).expect_err("an unclassifiable parameter is rejected");

        assert_eq!(error_summary(&error), "unclassifiable-transition-parameter crate::BeginChat crate :: Speak");
    }

    #[test]
    fn propagates_a_missing_process_method() {
        let source = format!(
            "{STORYBOARD_STATES}\n#[singleton]\n#[websocket_transition(from = crate::Fresh, on = crate::Speak)]\nstruct NoProcess;\n"
        );

        let error = scan_transitions(&source).expect_err("a missing #[process] method is reported");

        assert_eq!(error_summary(&error), "injection");
    }

    fn protocol_error(lib_source: &str) -> WebsocketCodegenError {
        let index = index_for(lib_source);
        let states = websocket_states(&index).expect("the states are scanned");
        let messages = websocket_messages(&index).expect("the messages are scanned");
        let events = websocket_internal_events(&index).expect("the events are scanned");
        let transitions = websocket_transitions(&index, &states, &messages, &events)
            .expect("the transitions are scanned");

        protocols(&states, &transitions).expect_err("the protocol analysis fails")
    }

    const VALID_PROTOCOL: &str = "\
#[websocket_state(server = \"public\", path = \"/storyboard\")]
struct Fresh;

#[websocket_state]
struct Thinking;

#[websocket_state(terminal)]
struct Ended;

#[websocket_message(method = \"speak\")]
struct Speak;

#[websocket_internal_event]
struct Done;

#[singleton]
#[websocket_transition(from = crate::Fresh, on = crate::Speak)]
struct Begin;

impl Begin {
    #[process]
    fn on(&self) -> crate::Thinking {}
}

#[singleton]
#[websocket_transition(from = crate::Thinking, on = crate::Done)]
struct Finish;

impl Finish {
    #[process]
    fn on(&self) -> crate::Ended {}
}
";

    #[test]
    fn groups_reachable_states_and_transitions_into_a_protocol() {
        let index = index_for(VALID_PROTOCOL);
        let states = websocket_states(&index).expect("the states are scanned");
        let messages = websocket_messages(&index).expect("the messages are scanned");
        let events = websocket_internal_events(&index).expect("the events are scanned");
        let transitions = websocket_transitions(&index, &states, &messages, &events)
            .expect("the transitions are scanned");

        let analyzed = protocols(&states, &transitions).expect("the protocol is analyzed");

        assert_eq!(analyzed.len(), 1);

        let protocol = &analyzed[0];

        assert_eq!(protocol.entry.to_string(), "crate::Fresh");
        assert_eq!(protocol.server, "public");
        assert_eq!(protocol.path, "/storyboard");
        assert_eq!(protocol.states.len(), 3);
        assert_eq!(protocol.transitions.len(), 2);
    }

    #[test]
    fn rejects_a_model_without_an_entry_state() {
        let error = protocol_error(
            "#[websocket_state]\nstruct Fresh;\n\n#[websocket_state(terminal)]\nstruct Ended;\n\n#[websocket_message(method = \"m\")]\nstruct Speak;\n\n#[singleton]\n#[websocket_transition(from = crate::Fresh, on = crate::Speak)]\nstruct Begin;\n\nimpl Begin {\n    #[process]\n    fn on(&self) -> crate::Ended {}\n}\n",
        );

        assert_eq!(error_summary(&error), "missing-entry-state");
    }

    #[test]
    fn rejects_two_entry_states_sharing_a_route() {
        let error = protocol_error(
            "#[websocket_state(server = \"public\", path = \"/x\")]\nstruct First;\n\n#[websocket_state(server = \"public\", path = \"/x\")]\nstruct Second;\n",
        );

        assert_eq!(error_summary(&error), "duplicate-protocol-route public /x");
    }

    #[test]
    fn rejects_a_terminal_state_with_an_outgoing_transition() {
        let error = protocol_error(
            "#[websocket_state(server = \"public\", path = \"/x\")]\nstruct Fresh;\n\n#[websocket_state(terminal)]\nstruct Ended;\n\n#[websocket_message(method = \"m\")]\nstruct Speak;\n\n#[singleton]\n#[websocket_transition(from = crate::Ended, on = crate::Speak)]\nstruct Bad;\n\nimpl Bad {\n    #[process]\n    fn on(&self) -> crate::Fresh {}\n}\n",
        );

        assert_eq!(error_summary(&error), "terminal-state-has-transitions crate::Ended crate::Speak");
    }

    #[test]
    fn rejects_an_unreachable_state() {
        let error = protocol_error(
            "#[websocket_state(server = \"public\", path = \"/x\")]\nstruct Fresh;\n\n#[websocket_state(terminal)]\nstruct Ended;\n\n#[websocket_state]\nstruct Orphan;\n\n#[websocket_message(method = \"m\")]\nstruct Speak;\n\n#[singleton]\n#[websocket_transition(from = crate::Fresh, on = crate::Speak)]\nstruct Begin;\n\nimpl Begin {\n    #[process]\n    fn on(&self) -> crate::Ended {}\n}\n",
        );

        assert_eq!(error_summary(&error), "unreachable-state crate::Orphan");
    }

    #[test]
    fn rejects_a_dead_end_state() {
        let error = protocol_error(
            "#[websocket_state(server = \"public\", path = \"/x\")]\nstruct Fresh;\n\n#[websocket_state]\nstruct Stuck;\n\n#[websocket_state(terminal)]\nstruct Ended;\n\n#[websocket_message(method = \"go\")]\nstruct Go;\n\n#[websocket_message(method = \"wander\")]\nstruct Wander;\n\n#[singleton]\n#[websocket_transition(from = crate::Fresh, on = crate::Go)]\nstruct Finish;\n\nimpl Finish {\n    #[process]\n    fn on(&self) -> crate::Ended {}\n}\n\n#[singleton]\n#[websocket_transition(from = crate::Fresh, on = crate::Wander)]\nstruct Stray;\n\nimpl Stray {\n    #[process]\n    fn on(&self) -> crate::Stuck {}\n}\n",
        );

        assert_eq!(error_summary(&error), "dead-end-state crate::Stuck");
    }

    const RENDER_PROTOCOL: &str = "\
#[websocket_state(server = \"public\", path = \"/storyboard\")]
struct Fresh;

#[websocket_state]
struct Thinking;

#[websocket_state(terminal)]
struct Ended;

#[websocket_message(method = \"conversation.speak\")]
struct Speak;

#[websocket_message(method = \"assistant.accepted\")]
struct Accepted;

#[websocket_internal_event]
struct GenerationComplete;

#[singleton]
#[websocket_transition(from = crate::Fresh, on = crate::Speak, emits(crate::Accepted))]
struct BeginChat;

impl BeginChat {
    #[process]
    fn on(
        &self,
        state: crate::Fresh,
        message: margaret_websocket::envelope::Envelope<crate::Speak>,
        emit: &margaret_websocket::emit::Emit<crate::Alphabet>,
        spawner: &margaret_websocket::activity_spawner::ActivitySpawner<crate::Internal>,
    ) -> crate::Thinking {}
}

#[singleton]
#[websocket_transition(from = crate::Thinking, on = crate::GenerationComplete)]
struct Complete;

impl Complete {
    #[process]
    fn on(&self, state: crate::Thinking, event: crate::GenerationComplete) -> crate::Ended {}
}
";

    fn formatted_module(artifacts: &WebsocketArtifacts, name: &str) -> String {
        artifacts
            .modules
            .iter()
            .find(|module| module.name() == name)
            .expect("the module is generated")
            .to_source()
    }

    #[test]
    fn generates_a_protocol_module_and_a_synthetic_route() {
        let artifacts =
            render_websocket(&index_for(RENDER_PROTOCOL)).expect("the protocol is generated");

        let module_names: Vec<&str> = artifacts
            .modules
            .iter()
            .map(margaret_generated_module::generated_module_tokens::GeneratedModuleTokens::name)
            .collect();

        assert!(module_names.contains(&"websocket"));
        assert!(module_names.contains(&"websocket/fresh"));

        let protocol_source = formatted_module(&artifacts, "websocket/fresh");

        assert!(protocol_source.contains("pub enum ProtocolState"));
        assert!(protocol_source.contains("pub enum Internal"));
        assert!(protocol_source.contains("BeginChatEmit"));
        assert!(protocol_source.contains("pub struct Dispatcher"));
        assert!(protocol_source.contains("async fn dispatch"));
        assert!(protocol_source.contains("async fn dispatch_internal"));
        assert!(protocol_source.contains("\"conversation.speak\""));

        let parent_source = formatted_module(&artifacts, "websocket");

        assert!(parent_source.contains("pub mod fresh"));
        assert!(parent_source.contains("WebsocketOutbound for crate :: Accepted"));

        assert_eq!(artifacts.synthetic_routes.len(), 1);

        let route = &artifacts.synthetic_routes[0];

        assert_eq!(route.server, "public");
        assert_eq!(route.path, "/storyboard");
        assert_eq!(route.label, "websocket handshake for /storyboard");
        assert!(
            route
                .handler
                .to_string()
                .contains("super :: super :: websocket :: fresh :: build")
        );
    }

    #[test]
    fn generated_protocol_module_is_a_valid_rust_file() {
        let artifacts =
            render_websocket(&index_for(RENDER_PROTOCOL)).expect("the protocol is generated");

        for module in artifacts.modules {
            module
                .format()
                .expect("each generated module is a valid Rust file");
        }
    }

    const SPARSE_BINDINGS_PROTOCOL: &str = "\
#[websocket_state(server = \"public\", path = \"/sparse\")]
struct Start;

#[websocket_state]
struct Middle;

#[websocket_state(terminal)]
struct Done;

#[websocket_message(method = \"open\")]
struct Open;

#[websocket_internal_event]
struct Ready;

#[singleton]
#[websocket_transition(from = crate::Start, on = crate::Open)]
struct Begin;

impl Begin {
    #[process]
    fn on(
        &self,
        facts: &margaret_websocket::connection_facts::ConnectionFacts,
        emit: &margaret_websocket::emit::Emit<crate::BeginAlphabet>,
    ) -> crate::Middle {}
}

#[singleton]
#[websocket_transition(from = crate::Middle, on = crate::Ready)]
struct Settle;

impl Settle {
    #[process]
    fn on(
        &self,
        emit: &margaret_websocket::emit::Emit<crate::SettleAlphabet>,
        spawner: &margaret_websocket::activity_spawner::ActivitySpawner<crate::Internal>,
    ) -> crate::Done {}
}
";

    const MINIMAL_PROTOCOL: &str = "\
#[websocket_state(server = \"public\", path = \"/minimal\")]
struct Enter;

#[websocket_state(terminal)]
struct Leave;

#[websocket_message(method = \"advance\")]
struct Advance;

#[singleton]
#[websocket_transition(from = crate::Enter, on = crate::Advance)]
struct Step;

impl Step {
    #[process]
    fn on(&self, state: crate::Enter) -> crate::Leave {}
}
";

    #[test]
    fn generates_valid_code_for_sparsely_bound_transitions() {
        let artifacts = render_websocket(&index_for(SPARSE_BINDINGS_PROTOCOL))
            .expect("the sparse protocol is generated");

        for module in artifacts.modules {
            module
                .format()
                .expect("each generated module is a valid Rust file");
        }
    }

    #[test]
    fn generates_valid_code_for_a_protocol_without_internal_events() {
        let artifacts = render_websocket(&index_for(MINIMAL_PROTOCOL))
            .expect("the minimal protocol is generated");

        for module in artifacts.modules {
            module
                .format()
                .expect("each generated module is a valid Rust file");
        }
    }

    fn render_error(lib_source: &str) -> WebsocketCodegenError {
        render_websocket(&index_for(lib_source)).expect_err("the generation fails")
    }

    #[test]
    fn render_propagates_a_state_error() {
        assert_eq!(error_summary(&render_error("#[websocket_state(server = 5)]\nstruct Fresh;\n")), "index");
    }

    #[test]
    fn render_propagates_a_message_error() {
        assert_eq!(error_summary(&render_error(
                "#[websocket_state(server = \"public\", path = \"/x\")]\nstruct Fresh;\n\n#[websocket_state(terminal)]\nstruct Ended;\n\n#[websocket_message(method = 5)]\nstruct M;\n",
            )), "index");
    }

    #[test]
    fn render_propagates_an_internal_event_error() {
        assert_eq!(error_summary(&render_error(
                "#[websocket_state(server = \"public\", path = \"/x\")]\nstruct Fresh;\n\n#[websocket_state(terminal)]\nstruct Ended;\n\n#[websocket_internal_event]\nenum E {}\n",
            )), "internal-event-not-a-struct crate::E");
    }

    #[test]
    fn render_propagates_a_transition_error() {
        assert_eq!(error_summary(&render_error(
                "#[websocket_state(server = \"public\", path = \"/x\")]\nstruct Fresh;\n\n#[websocket_state(terminal)]\nstruct Ended;\n\n#[websocket_message(method = \"m\")]\nstruct M;\n\n#[singleton]\n#[websocket_transition(from = crate::Missing, on = crate::M)]\nstruct T;\n\nimpl T {\n    #[process]\n    fn on(&self) -> crate::Ended {}\n}\n",
            )), "from-not-a-state crate::T crate :: Missing");
    }

    #[test]
    fn render_propagates_a_graph_error() {
        assert_eq!(error_summary(&render_error(
                "#[websocket_state]\nstruct Fresh;\n\n#[websocket_state(terminal)]\nstruct Ended;\n\n#[websocket_message(method = \"m\")]\nstruct M;\n\n#[singleton]\n#[websocket_transition(from = crate::Fresh, on = crate::M)]\nstruct T;\n\nimpl T {\n    #[process]\n    fn on(&self) -> crate::Ended {}\n}\n",
            )), "missing-entry-state");
    }

    #[test]
    fn rejects_an_unresolvable_transition_parameter() {
        let source = format!(
            "{STORYBOARD_STATES}\n#[singleton]\n#[websocket_transition(from = crate::Fresh, on = crate::Speak)]\nstruct T;\n\nimpl T {{\n    #[process]\n    fn on(&self, junk: NoSuchType) -> crate::Thinking {{}}\n}}\n"
        );

        assert_eq!(
            error_summary(&scan_transitions(&source).expect_err("an unresolvable parameter is rejected")),
            "unclassifiable-transition-parameter crate::T NoSuchType"
        );
    }

    #[test]
    fn rejects_an_unclassifiable_internal_transition_parameter() {
        let source = format!(
            "{STORYBOARD_STATES}\n#[singleton]\n#[websocket_transition(from = crate::Thinking, on = crate::GenerationComplete)]\nstruct T;\n\nimpl T {{\n    #[process]\n    fn on(&self, junk: crate::Speak) -> crate::Fresh {{}}\n}}\n"
        );

        assert_eq!(
            error_summary(&scan_transitions(&source).expect_err("an unclassifiable internal parameter is rejected")),
            "unclassifiable-transition-parameter crate::T crate :: Speak"
        );
    }

    #[test]
    fn rejects_a_state_reachable_from_two_entries() {
        let error = protocol_error(
            "#[websocket_state(server = \"public\", path = \"/a\")]\nstruct EntryA;\n\n#[websocket_state(server = \"public\", path = \"/b\")]\nstruct EntryB;\n\n#[websocket_state(terminal)]\nstruct Ended;\n\n#[websocket_message(method = \"x\")]\nstruct X;\n\n#[websocket_message(method = \"y\")]\nstruct Y;\n\n#[singleton]\n#[websocket_transition(from = crate::EntryA, on = crate::X)]\nstruct FromA;\n\nimpl FromA {\n    #[process]\n    fn on(&self) -> crate::Ended {}\n}\n\n#[singleton]\n#[websocket_transition(from = crate::EntryB, on = crate::Y)]\nstruct FromB;\n\nimpl FromB {\n    #[process]\n    fn on(&self) -> crate::Ended {}\n}\n",
        );

        assert_eq!(error_summary(&error), "ambiguous-protocol-membership crate::Ended");
    }

    const DIAMOND_PROTOCOL: &str = "\
#[websocket_state(server = \"public\", path = \"/diamond\")]
struct A;

#[websocket_state]
struct B;

#[websocket_state]
struct C;

#[websocket_state(terminal)]
struct D;

#[websocket_message(method = \"left\")]
struct Left;

#[websocket_message(method = \"right\")]
struct Right;

#[websocket_message(method = \"close_left\")]
struct CloseLeft;

#[websocket_message(method = \"close_right\")]
struct CloseRight;

#[websocket_message(method = \"shared\")]
struct Shared;

#[singleton]
#[websocket_transition(from = crate::A, on = crate::Left, emits(crate::Shared))]
struct GoLeft;

impl GoLeft {
    #[process]
    fn on(&self, emit: &margaret_websocket::emit::Emit<crate::L>) -> crate::B {}
}

#[singleton]
#[websocket_transition(from = crate::A, on = crate::Right)]
struct GoRight;

impl GoRight {
    #[process]
    fn on(&self) -> crate::C {}
}

#[singleton]
#[websocket_transition(from = crate::B, on = crate::CloseLeft, emits(crate::Shared))]
struct CloseB;

impl CloseB {
    #[process]
    fn on(&self, emit: &margaret_websocket::emit::Emit<crate::R>) -> crate::D {}
}

#[singleton]
#[websocket_transition(from = crate::C, on = crate::CloseRight)]
struct CloseC;

impl CloseC {
    #[process]
    fn on(&self) -> crate::D {}
}
";

    #[test]
    fn generates_valid_code_for_a_diamond_protocol_with_a_shared_emit() {
        let artifacts =
            render_websocket(&index_for(DIAMOND_PROTOCOL)).expect("the diamond protocol is generated");

        for module in artifacts.modules {
            module
                .format()
                .expect("each generated module is a valid Rust file");
        }
    }

    #[test]
    fn scan_transitions_propagates_a_state_error() {
        assert_eq!(
            error_summary(
                &scan_transitions("#[websocket_state(server = 5)]\nstruct Fresh;\n")
                    .expect_err("a bad state stops the transition scan")
            ),
            "index"
        );
    }

    #[test]
    fn scan_transitions_propagates_a_message_error() {
        assert_eq!(
            error_summary(
                &scan_transitions("#[websocket_message(method = 5)]\nstruct M;\n")
                    .expect_err("a bad message stops the transition scan")
            ),
            "index"
        );
    }

    #[test]
    fn scan_transitions_propagates_an_internal_event_error() {
        assert_eq!(
            error_summary(
                &scan_transitions("#[websocket_internal_event]\nenum E {}\n")
                    .expect_err("a bad internal event stops the transition scan")
            ),
            "internal-event-not-a-struct crate::E"
        );
    }

    #[test]
    fn rejects_a_state_with_malformed_arguments() {
        assert_eq!(
            error_summary(
                &websocket_states(&index_for("#[websocket_state(= 5)]\nstruct Fresh;\n"))
                    .expect_err("malformed state arguments are rejected")
            ),
            "index"
        );
    }

    #[test]
    fn rejects_a_state_with_a_non_string_path() {
        assert_eq!(
            error_summary(
                &websocket_states(&index_for(
                    "#[websocket_state(server = \"x\", path = 5)]\nstruct Fresh;\n"
                ))
                .expect_err("a non-string path is rejected")
            ),
            "index"
        );
    }

    #[test]
    fn rejects_a_message_with_malformed_arguments() {
        assert_eq!(
            error_summary(
                &websocket_messages(&index_for("#[websocket_message(= 5)]\nstruct M;\n"))
                    .expect_err("malformed message arguments are rejected")
            ),
            "index"
        );
    }

    #[test]
    fn rejects_a_transition_with_malformed_arguments() {
        assert_eq!(
            error_summary(
                &scan_transitions("#[singleton]\n#[websocket_transition(= 5)]\nstruct T;\n")
                    .expect_err("malformed transition arguments are rejected")
            ),
            "index"
        );
    }

    #[test]
    fn rejects_a_transition_with_a_non_path_from() {
        assert_eq!(
            error_summary(
                &scan_transitions(
                    "#[singleton]\n#[websocket_transition(from = 5, on = crate::X)]\nstruct T;\n"
                )
                .expect_err("a non-path from is rejected")
            ),
            "index"
        );
    }

    #[test]
    fn rejects_a_transition_with_a_non_path_on() {
        assert_eq!(
            error_summary(
                &scan_transitions(
                    "#[singleton]\n#[websocket_transition(from = crate::X, on = 5)]\nstruct T;\n"
                )
                .expect_err("a non-path on is rejected")
            ),
            "index"
        );
    }

    #[test]
    fn rejects_a_transition_with_a_non_path_emit() {
        let source = format!(
            "{STORYBOARD_STATES}\n#[singleton]\n#[websocket_transition(from = crate::Fresh, on = crate::Speak, emits(5))]\nstruct T;\n\nimpl T {{\n    #[process]\n    fn on(&self) -> crate::Thinking {{}}\n}}\n"
        );

        assert_eq!(
            error_summary(&scan_transitions(&source).expect_err("a non-path emit is rejected")),
            "index"
        );
    }
}


