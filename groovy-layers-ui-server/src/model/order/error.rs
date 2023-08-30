use serde::Serialize;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, Serialize, strum_macros::AsRefStr)]
#[serde(tag = "type", content = "data")]
pub enum Error{
    PrintFailed(String),
    SlicingFailed(String),
    SerdeJson(String)
}

impl From<serde_json::Error> for Error {
	fn from(value: serde_json::Error) -> Self {
        Self::SerdeJson(value.to_string())
    }
}

impl core::fmt::Display for Error {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(fmt, "{self:?}")
    }
}

impl std::error::Error for Error{}