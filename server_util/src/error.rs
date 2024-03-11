#[derive(Debug, Clone)]
pub struct IterEndError;

impl IterEndError {
    pub fn new() -> Self {
        IterEndError { }
    }
}

impl std::fmt::Display for IterEndError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("IterEndError: Iterator ended unexpectedly."))
    }
}

impl std::error::Error for IterEndError {}