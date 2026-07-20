# Margaret WebSocket Support — FSM / Session-Typed Protocol Design

## Context

Margaret is a Rust reinvention of the PHP *Resonance* framework. Its defining trait: components are declared with proc-macro **attributes** (near-no-op markers), and a **build-time code generator** (`margaret_codegen`, invoked from a one-line consumer `build.rs`) parses the consumer crate's source with `syn` exactly once and emits a fully-wired, compile-time-verified DI container + HTTP router + console + views under a generated `margaret` umbrella module. Everything is verified at compile time; there is zero runtime reflection.

We need first-class **WebSocket** support. Rather than copy Resonance's approach (a central `JsonRPCMethod` enum registry + ad-hoc responders — the exact "growing central registry" Margaret exists to eliminate), we model the **entire per-route protocol as a finite state machine (session types / typestate)**. The set of legal messages — inbound *and* outbound — at any moment falls out of "which state are we in." This is the most Margaret-idiomatic answer possible: it pushes "zero ambiguity, verified at compile time" all the way into the conversation itself.

**Why an FSM (not flat handlers):** the driving use case is stateful, multi-turn conversation (a "storyboard"). Attempts to constrain individual outbound message types failed — a handler legitimately needs to emit several messages, some unrelated to the immediate request. The FSM resolves this: "what may be emitted" becomes a first-class, compile-time-checked property of a transition, not an escape hatch. A wrong-message-for-the-current-state becomes a precise typed error. And because a session-typed FSM **projects** to the client, we can later generate a typed client (e.g. TypeScript) where sending the wrong next message is a **client-side compile error** — server and client share one source of truth.

**Prior art grounding this design** (from a multi-agent research pass): the academic `typestate` crate's **graph-validation algorithm** (every state must be *productive* — a terminal is reachable from it — and *useful* — productive **and** reachable from the initial state; signature-inferred transition roles) is lifted directly for compile-time protocol verification. `statig`/Hoverbear's "own a heterogeneous state enum and swap it" is the storage model. Erlang `gen_statem` / XState give the async concurrency model (internal/self events, entry/exit activities, distinct-typed unforgeable internal events). Session-type projection (Scribble / multiparty session types, `rumpsteak`) is the basis for the deferred client generator.

**Intended outcome:** a coherent extension of Margaret — not a bolt-on — where WebSocket protocols are declared with attributes, verified at compile time (unreachable states, dead-ends, ambiguous transitions, illegal messages all become compile errors), and served over the existing hyper/trzcina transport with the WS handshake riding the *same* HTTP router + middleware.

---

## Core model

- **States are types** (`#[websocket_state]`), may carry data. The framework owns a generated transparent `enum ProtocolState { Fresh(Fresh), Chatting(Chatting), … }` and swaps the whole value on each transition (single owner, by value — no `Option`/`mem::replace` holes).
- **Transitions are handlers** (`#[websocket_transition]` on a `#[singleton]` struct with DI-injected fields). A transition consumes the typed from-state **by value**, receives the validated inbound message + connection facts + a typed emit handle, and **returns the next state by value**: `async fn on(...) -> Result<NextState, WebsocketError>`.
- **The protocol** is the connected component of the state/transition graph rooted at an **entry state**, identified by the entry `(server, path)`. Protocol membership is a *computed graph fact*, not a per-transition tag that can drift.
- **Wire envelope: JSON-RPC 2.0** (`{jsonrpc, method, id?, params}` → `result`/`error`). Inbound dispatch is `(current_state, method) → transition`. Illegal/unknown/invalid messages produce an **in-band JSON-RPC error frame** and the connection stays open.
- **Two identity layers already in Margaret still apply:** the WS handshake is an HTTP GET, so SPIFFE-mTLS peer identity + session/auth middleware run *before* the upgrade.

### Resolved decisions (D1–D5), with adversarial-review fixes folded in

- **D1 — one loop, one rule (concurrency).** A transition is a fast, run-to-completion `async fn` that MAY `await` only bounded work. Slow/streaming work is never awaited inline: the transition moves to an intermediate state (e.g. `Thinking`), **spawns an activity** holding a clone of the typed emit handle + a child `CancellationToken`, and returns immediately; the activity streams frames and, on completion, sends a **distinct-typed internal event** back into the loop that drives the follow-up transition. The simple case just spawns nothing. **One serialized driver per connection**; concurrency lives across connections and in spawned activities. Inbound frames arriving during an in-flight transition are held by **TCP-level FIFO backpressure** (the single-consumer loop reads one frame per iteration) — no arbitrary in-memory queue, never rejected, ordering preserved. **[Review H1]** the transition is dispatched *under* the cancellation token (`tokio::select!` between `connection_token.cancelled()` and the transition future), so a hung transition can never defeat shutdown.
- **D2 — return the next state by value; transitions are infallible.** `#[process] -> <NextState>` (a concrete `#[websocket_state]`, e.g. `-> Thinking`), **not** a `Result`. The next-state set is the single source of truth in the signature; no `to=` to drift; move semantics make the old state inaccessible. There is **no application error enum returned by transitions** (this keeps the crate to one error concern — see the error section): fallible work (an LLM call, a DB write) runs in a **spawned activity** that reports success *or* failure as **distinct internal events** driving declared transitions — e.g. `(Thinking, GenerationComplete) → Chatting` and `(Thinking, GenerationFailed) → Ended` — so error handling is modeled *explicitly in the protocol graph* rather than hidden in a `Result`. Reaching a `terminal` state makes the driver enqueue a normal Close after that transition's final frame and end the loop. **v1: single concrete return state only.** Branching (a `#[websocket_next]` enum of states) is **deferred** — it needs a `margaret_attributes` enum-variant-indexing generalization and is the heaviest slice. *(This adjudicates the one contradiction the review flagged between the two design halves: transitions return a state, never a `Result`.)*
- **D3 — outbound fully constrained, per transition. [Review H2]** Each transition declares `emits(...)`; codegen generates a **per-transition** sealed typed handle `Emit<BeginChatEmit>` (alphabet = *that transition's* declared outputs). A spawned activity clones the *transition's* `Emit`, so streaming stays constrained to the spawning transition's alphabet even after the machine advances. The route-wide `Outbound` union exists only inside the writer channel; producers never see it. Pushing an undeclared type does not compile — no free-push escape hatch.
- **D4 — internal events + timers are first-class.** The loop `select!`s over four sources: socket frames, the internal-event `mpsc`, timers (modeled as activities that push internal events), and cancellation (`biased`, cancellation first). `TProtocol::Internal` is a generated enum **never** produced by decoding a wire frame, so a client cannot forge `GenerationComplete`/`IdleTimeout`. Internal-event *completion* transitions are **in v1** (load-bearing for the LLM-streaming case). **[Review M5]** an internal event with no transition from the current state is **swallowed** (state unchanged) — never an error frame (it's the expected completion-after-barge-in race, and internal events aren't client-visible).
- **D5 — owned enum + facts beside it.** The driver owns `ProtocolState` by value; immutable `ConnectionFacts` (peer identity, remote addr, route params, negotiated subprotocol) live in a **separate** struct passed by shared ref. **[Review M7]** channel-sizing knobs are split out into a per-server `WebSocketChannelConfig`, not mixed into `ConnectionFacts` (different lifetime / reason-to-change → single-responsibility-grouping).

---

## Attribute surface

Five passthrough proc-macros added to `margaret_macros/src/lib.rs` (same shape as the existing `singleton`/`responds_to_http`). None strip parameter markers — transition args are classified **by type** (like `HttpInjectable`), not by markers.

```rust
#[proc_macro_attribute] pub fn websocket_state(_a, item) -> TokenStream { item }
#[proc_macro_attribute] pub fn websocket_message(_a, item) -> TokenStream { item }
#[proc_macro_attribute] pub fn websocket_internal_event(_a, item) -> TokenStream { item }
#[proc_macro_attribute] pub fn websocket_transition(_a, item) -> TokenStream { item }
// #[websocket_next]  — DEFERRED to a later milestone (branching)
```

Declarations (every referenced type carries its full path — framework rule):

```rust
#[websocket_state(server = "public", path = "/storyboard/{id}")]  // entry state carries the route (see Grouping)
struct Fresh;

#[websocket_state]
struct Thinking;                       // intermediate state entered while an activity streams

#[websocket_state]
struct Chatting { history: Vec<crate::storyboard::Turn> }

#[websocket_state(terminal)]           // connection may end here; MUST have no outgoing transitions
struct Ended;

// A wire message: serde::Deserialize + validator::Validate. [Review M6] wire label is EXPLICIT & REQUIRED
// (never derived from the struct name — that would leak a rename into the external contract and let two
//  same-named structs in different modules collide).
#[websocket_message(method = "conversation.message")]
struct ConversationMessageForm { #[validate(length(min = 1))] text: String }

// An internal (non-wire) event — unforgeable from the wire (distinct Rust type). No wire label.
#[websocket_internal_event]
struct GenerationComplete { reply: crate::storyboard::Turn }

// Outbound messages: serde::Serialize + explicit wire label.
#[websocket_message(method = "assistant.accepted")]
struct AssistantAccepted { turn_id: uuid::Uuid }

// A transition — lives on a #[singleton]; its fields are DI-injected by the container.
#[singleton]
#[websocket_transition(
    from = crate::storyboard::states::Fresh,                     // dispatch key (from-state), FULL path
    on   = crate::storyboard::forms::ConversationMessageForm,    // dispatch key (message), FULL path
    emits(crate::storyboard::out::AssistantAccepted),            // D3 outbound alphabet for THIS transition
)]
struct BeginChat { llm: std::sync::Arc<dyn crate::storyboard::Llm> }

impl BeginChat {
    #[process]  // reused verbatim from margaret_injection_codegen::process_method
    async fn on(
        &self,
        state: crate::storyboard::states::Fresh,                                              // == `from`, consumed
        message: margaret_websocket::envelope::Envelope<crate::storyboard::forms::ConversationMessageForm>, // == `on`
        emit: &margaret_websocket::emit::Emit<crate::margaret::websocket::storyboard::BeginChatEmit>,       // typed to emits(...)
        facts: &crate::margaret::websocket::storyboard::ConnectionFacts,
        spawner: &margaret_websocket::activity_spawner::ActivitySpawner<crate::margaret::websocket::storyboard::Internal>,
    ) -> crate::storyboard::states::Thinking {                    // infallible: the next state, by value
        emit.push(AssistantAccepted { turn_id }).await;          // best-effort; a dead channel just means teardown
        spawner.spawn({ let emit = emit.clone(); async move {
            // stream tokens via emit.push(..); on success send Internal::GenerationComplete, on failure GenerationFailed
        }});
        Thinking
    }
}

// Fallible work + error handling are modeled as declared internal-event transitions (no Result, no branching):
#[singleton] #[websocket_transition(from = crate::storyboard::states::Thinking,
    on = crate::storyboard::events::GenerationComplete, emits(crate::storyboard::out::AssistantReply))]
struct CompleteChat { /* … */ }   // (Thinking, GenerationComplete) -> Chatting
#[singleton] #[websocket_transition(from = crate::storyboard::states::Thinking,
    on = crate::storyboard::events::GenerationFailed, emits(crate::storyboard::out::GenerationError))]
struct FailChat { /* … */ }       // (Thinking, GenerationFailed) -> Ended (terminal ⇒ driver Closes)
```

**Protocol grouping — the answer.** A protocol is **not** an explicit user type. The route `(server, path)` is declared **once, on the entry state** (`#[websocket_state(server=…, path=…)]`), and the protocol is every state/transition reachable (forward) from that entry state. This keeps the route a single source of truth and makes membership a computed graph fact (no per-transition route tag to keep consistent). Codegen materializes it as one generated module `crate::margaret::websocket::<entry_state_field_name>` (name derived from the entry state's `CanonicalPath`, disambiguated by `NameAllocator`, exactly like `views`/`http`).

---

## Runtime — `margaret_websocket` crate

A single serialized `select!` loop, generic over a generated `Protocol` trait. New workspace dep: **`tokio-tungstenite = "=0.30"`** (MIT, ~222M downloads; `from_raw_socket` over `TokioIo<Upgraded>` + `tungstenite::handshake::derive_accept_key` for the 101). Dependency arrow is `margaret_websocket → margaret_http` only (no cycle; `margaret_http` gains zero WebSocket knowledge).

### The driver loop (`run_connection`)

Owns `ProtocolState` **by value**; threads it through each dispatch (no `Option`/`mem::replace`). Reads one frame per iteration; a slow transition simply stops reading (TCP backpressure = the bounded FIFO). **[Review H1]** the transition future is raced against the cancellation token so it can always be interrupted.

```rust
let mut state = protocol.initial_state(&facts);
let mut activities = spawner.on_entry(&protocol, &state, &facts, &emit);
loop {
    let event = tokio::select! {
        biased;
        () = connection_token.cancelled() => Event::Cancelled,
        internal = internal_rx.recv()      => Event::internal_or_closed(internal),
        frame = read.next()                => Event::from_read(frame),
    };
    match event {
        Event::Cancelled | Event::PeerClosed => break Ok(()),
        Event::Transport(e)                  => break Err(WebSocketConnectionError::Transport(e)),
        Event::Ping(p) => emit.enqueue(OutboundFrame::Pong(p)),
        Event::Close(_) => { emit.enqueue(OutboundFrame::Close(CloseFrame::normal())); break Ok(()); }
        Event::Wire(bytes) => match InboundFrame::decode(&bytes) {      // [Review M4] total data-classification, NOT a Result
            InboundFrame::ParseError        => emit.enqueue(JsonRpcErrorFrame::parse_error().into()),
            InboundFrame::InvalidEnvelope{id}=> emit.enqueue(JsonRpcErrorFrame::invalid_request(id).into()),
            InboundFrame::Request{method,params,id} | InboundFrame::Notification{method,params,..} => {
                state = tokio::select! {                                 // [Review H1] transition under the token
                    () = connection_token.cancelled() => break Ok(()),
                    d  = protocol.on_wire(state, &facts, &method, params.as_ref(), id, &emit, &spawner) => match d {
                        WireDispatch::Advanced(next) => { activities = activities.transition_to(&protocol, &next, &facts, &emit, &spawner); next }
                        WireDispatch::Reject{state, frame} => { emit.enqueue(frame.into()); state }
                    }
                };
            }
        },
        Event::Internal(internal) => {                                  // [Review M5] no-transition ⇒ swallowed (state unchanged)
            let next = protocol.on_internal(state, &facts, internal, &emit, &spawner).await;
            activities = activities.transition_to(&protocol, &next, &facts, &emit, &spawner);
            state = next;
        }
    }
}
activities.cancel().await;   // exit actions of the final state
drop(emit); writer.await…;   // last sender dropped → writer flushes + Close handshake
```

### The `Protocol` trait (generated per route)

```rust
#[async_trait] pub trait Protocol: Send + Sync + 'static {
    type State: Send;                       // generated ProtocolState enum
    type Internal: Send;                    // generated internal-event enum (unforgeable)
    fn initial_state(&self, facts: &ConnectionFacts) -> Self::State;
    async fn on_wire(&self, state: Self::State, facts: &ConnectionFacts, method: &str,
                     params: Option<&serde_json::Value>, request_id: Option<RequestId>,
                     emit: &EmitCore, spawner: &ActivitySpawner<Self::Internal>) -> WireDispatch<Self::State>;
    async fn on_internal(&self, state: Self::State, facts: &ConnectionFacts, event: Self::Internal,
                         emit: &EmitCore, spawner: &ActivitySpawner<Self::Internal>) -> Self::State;
}
pub enum WireDispatch<TState> { Advanced(TState), Reject { state: TState, frame: JsonRpcErrorFrame } }  // frame is DATA
```

Generated `on_wire`: `(state discriminant, method)` lookup in the compile-time table → unknown-to-route ⇒ `Reject(method_not_found -32601)`; known-but-not-from-this-state ⇒ `Reject(illegal_for_state -32000, data={current_state, method, allowed_methods})`; then `margaret_validation::validate_json::<Msg>(params)` → `Invalid`/`Malformed` ⇒ `Reject(invalid_params -32602)`; `Valid` ⇒ run transition ⇒ `Advanced(next)`. The state is threaded through unchanged on every reject.

### Outbound: bounded mpsc + single writer

- Untyped `EmitCore { tx: mpsc::Sender<OutboundFrame> }`; codegen wraps it per-transition in `Emit<BeginChatEmit>` (**[Review H2]**). `OutboundFrame` = `{ Result{id,payload} | Notify{method,params} | Error(JsonRpcErrorFrame) | Pong | Close }` — all data.
- **One writer task** owns the split `SplitSink`, drains the bounded channel FIFO (serializing all concurrent producers), cancels `connection_token` on a dead socket, flushes+closes when all senders drop. **[Review L9/L11]** outbound serialization failures are total and non-panicking (→ `-32603`, never `unwrap`); sink failures fold into the single `WebSocketConnectionError`.
- **Backpressure:** bounded channel (`outbound_capacity`, a per-server `WebSocketChannelConfig` value grounded in the deployment's memory/frame budget — never a magic constant). `emit.push` awaits when full → a fast producer is slowed to the socket's drain rate. Liveness is the ping/pong heartbeat activity (interval + missed-pong threshold from per-server config, grounded in the proxy idle timeout — no arbitrary inline timeout).

### Internal events, timers, activities (D4)

`ActivitySpawner::on_entry` spawns a state's declared activities/timers under a fresh child of `connection_token`. `transition_to` cancels the previous state's `ActivityHandles` (awaiting them) *before* arming the next state's — "started-on-entry, cancelled-on-exit". A timer is just `spawner.timer(after, || Internal::IdleTimeout)`. Activities own only a cloned `Emit` + child token + `internal_tx` clone — **never** the state.

### JSON-RPC error frames (data, connection stays open)

| Situation | Detected in | Code |
|---|---|---|
| Not parseable JSON | runtime `InboundFrame::decode` | **-32700** |
| Not a valid JSON-RPC 2.0 request/notification | runtime decode | **-32600** |
| Method unknown to the whole route | generated `on_wire` | **-32601** |
| Method known but illegal from current state | generated `on_wire` | **-32000** (server-reserved; `data={current_state, method, allowed_methods}`) |
| `params` fail validation | `validate_json` in `on_wire` | **-32602** |
| Activity panicked (`JoinError`) or outbound serialize failed | supervisor / writer | **-32603** |

The crate's single `thiserror` enum `WebSocketConnectionError` is a *different* concern (fatal driver I/O): `Transport(#[from] tungstenite::Error)`, `Writer(JoinError)`, `Upgrade(#[source] hyper::Error)`, `Sink(#[source] …)`. It never carries a wire frame. **[Review L10]** v1 scopes to HTTP/1.1 upgrades (RFC 6455); h2/RFC 8441 is out of scope.

---

## Codegen — `margaret_websocket_codegen` crate

Sibling of `margaret_service_codegen`; depends on `margaret_attributes`, `margaret_injection_codegen`, `margaret_http_codegen`, `margaret_codegen_tokens`, `margaret_generated_module` (no cycle; WS sits above HTTP). Entry mirrors `render_http`:

```rust
pub fn render_websocket(index: &AttributeIndex, has_views: bool) -> Result<WebsocketArtifacts, WebsocketCodegenError>
```
`WebsocketArtifacts` carries `Vec<GeneratedModuleTokens>` **and** `Vec<SyntheticRoute>` (handshake GET routes to fold into the HTTP table).

- **Scanning** mirrors `views(index)` / `http_routes`: `AttributeSelector::from_marker("websocket_state"|"websocket_message"|"websocket_internal_event"|"websocket_transition")` → `index.select`; struct-only via `struct_identifier`; resolve `from`/`on`/`emits(...)` type paths with `resolve_item_path` to **canonical paths** (never bare names — two same-named structs coexist); find the `#[process]` method via the reused `margaret_injection_codegen::process_method`.
- **Argument injection** — new `WebsocketInjectable` enum mirrors `http_injectable.rs`: classify each `parameters(sig)` entry by resolved type — from-state (by value), trigger (`Envelope<on>` / bare internal event), `&Emit<…>`, `&ConnectionFacts`, `&ActivitySpawner<…>`, `CancellationToken` (reuse `is_cancellation_token`). Transition struct fields are DI-provided (`#[singleton]`, `container.<field>().await`).
- **Dispatch table** — mirrors `HttpRouteTable`/`ServerRouteGroup`, keyed by `(from_state_canonical_path, on_canonical_path)`; `insert` collision ⇒ `DuplicateTransition`. Plus per-state wire-label uniqueness ⇒ `DuplicateWireMethod`.
- **Protocol grouping + graph analysis** — entry states = `from` of route-bearing entry states; forward BFS over transition edges → reachable set = the protocol; a state reachable from two entries ⇒ `AmbiguousProtocolMembership`; from none ⇒ `UnreachableState`.
- **Generated per-protocol code** (transparent-type pattern, mirrors `margaret_views_codegen`): `ProtocolState` transparent enum (variant name == state type name) + `From` impls; `ConnectionFacts` struct; per-transition sealed `BeginChatEmit` + the protocol `Outbound` union (writer-only); the `Protocol` impl dispatcher as an **exhaustive `match` over `ProtocolState`** (compiler-checked totality → no `unreachable!()`, honoring "generated code never panics").
- **Fold the handshake GET into the shared HTTP route table** — emit a `SyntheticRoute { server, path, method:"GET", handler_tokens, middleware_tags, requires_peer_spiffe_id }`; `margaret_codegen` hands it to the HTTP pass, which inserts it into the **one** `HttpRouteTable` → existing `DuplicateRoute`/`ConflictingRoutePaths` fire across HTTP+WS uniformly, and `active_servers`/`server_module` render it as a normal `RouteEntry`.

### Compile-time checks (every one a typed `WebsocketCodegenError` variant, one enum per crate)

`DuplicateTransition{from,on,first,second}` · `DuplicateWireMethod{from,method,…}` · `MissingEntryState{server,path}` / `InconsistentProtocolRoute{…}` · `UnreachableState{state}` (useful) · `DeadEndState{state}` (productive; non-terminal with no outgoing) · `TerminalStateHasTransitions{state,on}` · `FromNotAState` / `OnNotAMessageOrEvent` · `NextNotAState{transition}` (from `signature().output`) · `WebsocketMessageNotAStruct` (+ Deserialize/Validate backstopped by generated `validate_json::<T>` compiling) · `UnclassifiableTransitionParameter` · `StateParameterMismatch{from,written}` · `AmbiguousProtocolMembership` · `TransitionNotSingleton`/`TransitionNotAStruct` · `Index{#[from] AttributeError}`. Reachability is the textbook two-pass (typestate *useful*+*productive*) over the expanded graph.

---

## Generalizations of existing crates (nothing else is touched)

1. **`margaret_http` transport seam (WS-agnostic — no tungstenite):**
   - New `src/upgrade_handler.rs` → `trait UpgradeHandler { fn switching_response(&self) -> Response; async fn serve(self: Box<Self>, upgraded: TokioIo<Upgraded>, connection_token: CancellationToken); }` and `ResponseContinuation::Upgrade(Box<dyn UpgradeHandler>)` (currently `Done|Forward|Redirect`).
   - Widen the terminal from `Response` to `ServedOutcome { Http(Response), Upgrade(Box<dyn UpgradeHandler>) }` through `respond_recursively`→`Router::respond`→`dispatch`. Forward/Redirect still resolve to `Http` inside the recursion; only a terminal `Upgrade` bubbles out.
   - `dispatch` (`bound_server.rs:244`): call `hyper::upgrade::on(&mut request)` at the top (before `into_parts` drops `parts.extensions` at line 255), route, then on `Upgrade` return the 101 (`switching_response`) and `tokio::spawn` a task awaiting the captured `OnUpgrade` → `handler.serve(TokioIo::new(upgraded), child_token)`; on `Http`, drop the unused `OnUpgrade`.
   - `serve_connection` → `serve_connection_with_upgrades` (`bound_server.rs:225`; `UpgradeableConnection: GracefulConnection`, existing `watcher.watch` still governs the HTTP side).
   - `ConnectionContext` (`bound_server.rs:43`, no token today) gains the server `CancellationToken` threaded from `serve` (line 99); `spawn_connection` derives per-connection `child_token()`; the upgrade derives a further child for `run_connection`. Server shutdown cascades: server → connection → driver → activities. `serve()` tracks driver tasks in a `TaskTracker` and `wait().await`s after `graceful.shutdown()` so live WS loops aren't dropped mid-frame (hyper graceful can't reach a post-upgrade socket). **[Review H3]** the generated handshake `Handler` **validates** `Upgrade`/`Sec-WebSocket-Key`/`Version:13` and returns a plain `426`/`400` `Response` when absent — `ResponseContinuation::Upgrade` only for a valid handshake — so a stray plain GET never yields a bogus 101 + leaked task.
2. **`margaret_http_codegen`:** new `src/synthetic_route.rs` → `pub struct SyntheticRoute`; `HttpRoute` gains a handler-source (existing responder wiring **or** supplied `TokenStream`); `render_http`/`http_routes` accept `&[SyntheticRoute]` and resolve their middleware tags against the existing `middleware_plans` (middleware resolution stays solely here). Crate stays WS-agnostic (opaque handler tokens).
3. **`margaret_codegen`:** `capabilities.rs` add `has_websocket` (via `margaret_websocket_codegen::has_websocket`), `serves = has_http || has_services || has_websocket`; new `src/websocket_pass.rs` (runs after `views_pass`, before `http_pass`); `build_context.rs` carries `websocket_synthetic_routes`; `http_pass.rs` gate becomes `has_http || has_websocket` and forwards the synthetic routes; `umbrella.rs` gates the server modules on `has_http || has_websocket` and adds `pub mod websocket;`; `build.rs` order `container → views → websocket → http → services → console`; `codegen_error.rs` adds `Websocket{#[from] WebsocketCodegenError}`.
4. **`margaret_attributes` (DEFERRED, only for branching):** index enum variants (`IndexedVariant`, `IndexedItem.variants`, `module_walker` `Item::Enum`) — needed only for the deferred `#[websocket_next]` branch enums. `IndexedItem` has no `variants` field today (verified).

All other reuse is **direct**: `process_method` / `parameters` / `is_cancellation_token`; `AttributeSelector`/`AttributeQuery`/`resolve_item_path`/`struct_identifier`/`CanonicalPath`/`NameAllocator`; the route-table collision pattern; the Views transparent-type pattern; `GeneratedModuleTokens`/`format()`; `margaret_validation::validate_json`.

---

## Client generation (session-type projection) — designed-for, emitter deferred

Because the protocol is a session-typed FSM, it **projects** to the client endpoint: from the one compiled protocol we can emit a typed client (TypeScript) where the current state only exposes the messages legal in that state, so sending the wrong next message is a **client-side compile error**, not merely a server-side runtime rejection. Server and client then share one source of truth. The v1 architecture is designed so projection is possible (states, transitions, wire labels, and the `(state,method)→next` graph are all first-class compile-time data); the actual TS emitter is a **later milestone**, not v1.

---

## Minimal coherent v1 (and what's deferred)

**In v1** (the irreducible core honoring "concurrent, not sequential"):
1. `#[websocket_state]`, `#[websocket_message]`, `#[websocket_transition]`, `#[websocket_internal_event]` — **no** `#[websocket_next]`.
2. Single-concrete-return transitions — return one concrete state **by value** (infallible; errors are modeled as internal-event transitions, not a `Result`).
3. Wire transitions **and** internal-event completion transitions (internal events are how spawned heavy work rejoins without serializing the machine — load-bearing for the LLM-streaming flagship).
4. Per-transition typed `Emit` [H2]; one writer task + bounded mpsc; connection child-token teardown + `TaskTracker` at shutdown; transition raced against cancellation [H1].
5. JSON-RPC framing with in-band error frames (decode as data-classification [M4]); connection stays open; illegal-message-for-state ⇒ `-32000`.
6. Handshake header validation [H3]; explicit required wire `method` [M6].

**Deferred** (gold-plating for v1): `#[websocket_next]` branching + `margaret_attributes` variant-indexing; a per-state entry/exit auto-spawn *activity framework* (v1 = explicit `spawner.spawn(...)` + connection-token teardown + swallow stale completions); standalone timers beyond a connection-level idle-timeout; subprotocol negotiation; the generated TypeScript client.

---

## Milestones (each independently compilable + testable; no mocks — real indices, real generated-code formatting, real WS client)

- **M1 — passthrough macros.** Add the 4 attributes to `margaret_macros`; unit tests (no-op / no strip).
- **M2 — `margaret_http` transport seam.** `upgrade_handler.rs`, `ResponseContinuation::Upgrade`, `ServedOutcome` through `respond_recursively`/`Router::respond`/`dispatch`, `OnUpgrade` capture, `serve_connection_with_upgrades`, `ConnectionContext` child token, `TaskTracker`. Test: an in-crate `UpgradeHandler` doing a trivial echo; a raw client performs the handshake, asserts `101`, echoes one frame; a non-upgrade GET to the same path gets `426` (H3); existing `serves_connections_until_cancellation` still green.
- **M3 — `margaret_websocket` runtime.** `Envelope`, `InboundFrame` (data-classification), `EmitCore`+typed `Emit`, `writer_task`, `ActivitySpawner`+internal events, `Protocol`/`WireDispatch`, `run_connection`, `JsonRpcErrorFrame`, `WebSocketConnectionError`. Test against a hand-written fixture `Protocol`: validate→dispatch; unknown-method → `-32601` + connection open; illegal-for-state → `-32000`; invalid-params → `-32602`; cancellation tears down; a spawned activity streams then completes via an internal event.
- **M4 — `margaret_websocket_codegen`.** IR + renderers + `WebsocketCodegenError`; entry `render_websocket`; `has_websocket`. Test standalone (no HTTP folding yet): assert on formatted module output and on each error variant (duplicate transition, unreachable state, dead-end, illegal terminal, inconsistent route, unclassifiable parameter, from/on-not-a-state).
- **M5 — `margaret_http_codegen` + `margaret_codegen` wiring.** `SyntheticRoute` input + `HttpRoute` handler-source; capabilities/passes/umbrella/error generalizations. Test: `generate(...)` on a WS crate emits `pub mod websocket;` + a `server_public` registering a GET handler into `websocket::…`; HTTP-GET + WS on one path ⇒ `already registered` compile error; a WS-only server still generates server/routes/CLI-addr modules.
- **M6 — integration + example.** New `margaret_websocket_tests` crate (real fixture server; connect a real `tokio-tungstenite` client; drive `Fresh → Thinking → Chatting`; assert the `-32000` frame for an illegal-for-state message; assert connection stays open; assert cancellation teardown). Add a `storyboard` protocol to `margaret_example` (excluded from coverage per CLAUDE.md).

---

## Verification

- Per-crate `cargo test` for each milestone (codegen crates use the existing tempdir→`AttributeIndex`→render→`format()` harness; runtime + integration drive a real WS client end-to-end — no mocks).
- `margaret_example` must build and `cargo run -- serve` must boot the `public` server hosting both HTTP routes and the `/storyboard/{id}` WS route; the integration test drives the full `Fresh → Thinking → Chatting` flow, the illegal-message error frame, connection persistence, and clean shutdown on token cancel.
- Every compile-time check has a dedicated failing-fixture test asserting the exact `WebsocketCodegenError` variant.
- `make` targets (clippy / rust tests) stay separate per repo rules; new crates added to the workspace + CI matrices.

---

## Adversarial-review status

**Verdict: sound to plan from.** The spine — single-owner typestate machine, spawned heavy work rejoining via distinct-typed internal events, per-transition compile-time-constrained outbound, WS routes folded into the one HTTP route table, bolt-on purely via `ResponseContinuation::Upgrade` + `OnUpgrade` capture + `ConnectionContext` child token — is coherent, grounded in the real seams, and consistent with the framework's zero-ambiguity / compile-time / single-source-of-truth rules. All defects were at the edges and are folded into this plan: H1 (race transition against cancellation), H2 (per-transition typed `Emit`), H3 (validate handshake headers), M4 (decode as data, not an error enum), M5 (swallow stale internal events), M6 (explicit required wire `method`), M7 (split channel config out of facts), L9–L11 (non-panicking serialize path, h1-only, single sink error enum). Branching, timers, and the entry/exit activity framework are cut from v1.

## Open items to confirm at approval

- **Infallible transitions** — `#[process]` returns the next state by value, never a `Result` [D2]. Genuine fallible work happens in spawned activities that report success/failure as distinct internal events driving declared transitions. This keeps the crate to one error concern and models errors explicitly in the graph. Confirm you prefer this over fallible (`Result`-returning) transitions.
- **Explicit required wire `method`** on `#[websocket_message]` [M6] — a deliberate change from deriving it from the struct name, to keep the wire contract from riding on a rename. Confirm you're happy requiring it.
- **v1 scope** — defer branching (`#[websocket_next]`) / standalone timers / the entry-exit activity framework / the generated TS client. Adjust if you want any pulled into v1.
- On exit from plan mode, this document is copied verbatim into the repo (proposed `docs/websocket_fsm_design.md`).
