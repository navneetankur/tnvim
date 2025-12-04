use core::fmt::Display;
use std::borrow::Cow;
pub type Result<T> = core::result::Result<T, Error>;
#[derive(Debug)]
pub struct Error {
    msg: Cow<'static, str>,
    inner: Option<Box<dyn core::error::Error>>,
}
impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if !self.msg.is_empty() {
            f.write_str(&self.msg)?;
        }
        if let Some(inner) = &self.inner {
            if !self.msg.is_empty() {
                f.write_str(", caused by: ")?;
            }
            write!(f, "{inner}")?;
        }
        return Ok(());
    }
}
impl core::error::Error for Error{}

pub fn with_msg<T>(msg: impl Into<Cow<'static, str>>) -> Result<T> {
    Err(
        Error { msg: msg.into(), inner: None }
    )
}
pub fn with_inner<T>(inner: impl core::error::Error + 'static) -> Result<T> {
    Err(
        Error { msg: "".into(), inner: Some(Box::new(inner)) }
    )
}
pub fn with_msg_inner<T>(msg: impl Into<Cow<'static, str>>, inner: impl core::error::Error + 'static) -> Result<T> {
    Err(
        Error { msg: msg.into(), inner: Some(Box::new(inner)) }
    )
}
impl Error {
    pub fn from_inner(inner: impl core::error::Error + 'static) -> Self {
        Error { msg: "".into(), inner: Some(Box::new(inner)) }
    }
}

impl<T: 'static> From<tokio::sync::mpsc::error::SendError<T>> for Error {
    fn from(value: tokio::sync::mpsc::error::SendError<T>) -> Self {
        Self::from_inner(value)
    }
}
impl From<tokio::sync::oneshot::error::RecvError> for Error {
    fn from(value: tokio::sync::oneshot::error::RecvError) -> Self {
        Self::from_inner(value)
    }
}
impl From<rmp_serde::encode::Error> for Error {
    fn from(value: rmp_serde::encode::Error) -> Self {
        Self::from_inner(value)
    }
}
impl From<rmpv::ext::Error> for Error {
    fn from(value: rmpv::ext::Error) -> Self {
        Self::from_inner(value)
    }
}
