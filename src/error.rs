#[derive(Debug)]
pub enum Error {
    HeaderString(String),
    InvalidPageSize(String),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::HeaderString(v) => write!(f, "Unexpected bytes at start of file, expected the magic string 'SQLite format 3\u{0}', found {:?}", v),
            Self::InvalidPageSize(msg) => write!(f, "Invalid page size, {}", msg),
        }
    }
}

impl std::error::Error for Error {}