trait Greeter {}

#[provider(provides = Greeter)]
enum Bad {}
