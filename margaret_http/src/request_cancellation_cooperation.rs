#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RequestCancellationCooperation {
    Cooperative,
    Immediate,
}
