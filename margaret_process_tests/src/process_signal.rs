#[derive(Clone, Copy)]
pub enum ProcessSignal {
    Interrupt,
    Null,
    Terminate,
}

impl ProcessSignal {
    pub(crate) fn name(self) -> &'static str {
        match self {
            Self::Interrupt => "INT",
            Self::Null => "0",
            Self::Terminate => "TERM",
        }
    }
}
