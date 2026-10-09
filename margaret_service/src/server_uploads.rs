#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ServerUploads {
    Accepted { directory_argument: &'static str },
    Refused,
}
