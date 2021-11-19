use failure::Fail;
use std::io;

#[derive(Debug, Fail)]
pub enum LineSenderError {
    #[fail(display = "IO error: {}", _0)]
    Io(#[cause] io::Error),
    #[fail(display = "Serde error: {}", _0)]
    Serde(#[cause] serde_json::Error),
}

pub type Result<T> = std::result::Result<T, LineSenderError>;

impl From<io::Error> for LineSenderError {
    fn from(err: io::Error) -> LineSenderError {
        LineSenderError::Io(err)
    }
}

impl From<serde_json::Error> for LineSenderError {
    fn from(err: serde_json::Error) -> LineSenderError {
        LineSenderError::Serde(err)
    }
}
