use crate::clock::Clock;

pub struct SystemClock {
    revision: String,
}

impl SystemClock {
    pub fn new(revision: String) -> Self {
        Self { revision }
    }
}

impl Clock for SystemClock {
    fn revision(&self) -> String {
        self.revision.clone()
    }
}
