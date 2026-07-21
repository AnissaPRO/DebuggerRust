use std::io;

#[derive(Debug)]
pub enum LoggerError {
    Io(io::Error),

    Json(serde_json::Error),

    InvalidLog(String),
}

impl From<io::Error> for LoggerError {
    fn from(err: io::Error) -> Self {
        LoggerError::Io(err)
    }
}

impl From<serde_json::Error> for LoggerError {
    fn from(err: serde_json::Error) -> Self {
        LoggerError::Json(err)
    }
}
