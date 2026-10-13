use margaret_macros::process;

struct Twice;

impl Twice {
    #[process]
    #[process]
    fn run(&self) {}
}
