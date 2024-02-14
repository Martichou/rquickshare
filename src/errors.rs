#[derive(Debug)]
pub enum AppError {
    NotAnError,
}

impl std::fmt::Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NotAnError => write!(f, "not an error"),
        }
    }
}
