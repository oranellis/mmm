// Error and Result type

#[derive(Debug)]
pub enum MmmError {
    General(String),
    Io(std::io::Error),
}

impl std::fmt::Display for MmmError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            MmmError::General(s) => write!(f, "{}", s),
            MmmError::Io(err) => write!(f, "i/o error: {}", err),
        }
    }
}

impl std::error::Error for MmmError {}

impl From<std::io::Error> for MmmError {
    fn from(error: std::io::Error) -> Self {
        MmmError::Io(error)
    }
}

impl From<String> for MmmError {
    fn from(error: String) -> Self {
        MmmError::General(error)
    }
}

impl From<&str> for MmmError {
    fn from(error: &str) -> Self {
        MmmError::General(error.to_string())
    }
}

pub type MmmResult<T> = Result<T, MmmError>;
