/// Where a serve input comes from at runtime. A console command builds its container without the
/// serve prelude, so an input the prelude provisions cannot reach one.
#[derive(Debug, Eq, PartialEq)]
pub enum ServeInputProvisioning {
    EveryCommand,
    ServeCommandOnly,
}
