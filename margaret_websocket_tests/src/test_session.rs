use std::sync::Mutex;

#[derive(Default)]
pub struct TestSession {
    pub notifications: Mutex<Vec<String>>,
}
