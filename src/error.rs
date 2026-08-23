pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    StdIoError(std::io::Error),
    InvalidLine(String, String),
    NotEnoughParts,
    NoHome,
}

impl std::error::Error for Error {}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::StdIoError(e) => std::fmt::Display::fmt(e, f),
            Self::InvalidLine(line, errmsg) => write!(f, "parse line `{line}..` failed: {errmsg}"),
            Self::NotEnoughParts => write!(f, "not enough parts"),
            Self::NoHome => write!(f, "$HOME is not set"),
        }
    }
}
