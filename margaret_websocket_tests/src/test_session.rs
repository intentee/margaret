use std::sync::Mutex;

#[derive(Default)]
pub struct TestSession {
    pub cleanups: Mutex<Vec<String>>,
    pub notifications: Mutex<Vec<String>>,
}
