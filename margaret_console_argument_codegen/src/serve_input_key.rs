#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum ServeInputKey {
    ConsoleArgument { name: String },
    SpiffeHttpClient,
}
