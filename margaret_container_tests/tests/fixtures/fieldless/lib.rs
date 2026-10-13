use margaret::framework::macros::singleton;

#[singleton]
struct UnitMarker;

#[singleton]
struct EmptyNamed {}

#[singleton]
struct EmptyTuple();
