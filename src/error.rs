pub type Result<T> = std::result::Result<T, Error>;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("parse line `{line}..` failed: {errmsg}")]
    InvalidLine { line: String, errmsg: String },
    #[error("not enough parts")]
    NotEnoughParts,
    #[error("$HOME is not set")]
    NoHome,
}
