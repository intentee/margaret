trait Greeter {}

#[provider(provides = Greeter)]
struct Bad;
