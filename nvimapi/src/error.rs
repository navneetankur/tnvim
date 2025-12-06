use core::fmt::Display;
use std::borrow::Cow;

use rmpv::Value;
pub type Result<T> = core::result::Result<T, Error>;
#[derive(Debug)]
pub struct Error {
    pub msg: Cow<'static, str>,
    pub inner: Option<Box<dyn core::error::Error>>,
    pub inner_value: Option<Box<Value>>,
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
        if let Some(value) = &self.inner_value {
            write!(f, "value: {value}")?;
        }
        return Ok(());
    }
}
impl core::error::Error for Error{}

pub fn with_msg<T>(msg: impl Into<Cow<'static, str>>) -> Result<T> {
    Err(
        Error::from_msg(msg)
    )
}
pub fn with_inner<T>(inner: impl core::error::Error + 'static) -> Result<T> {
    Err(
        Error::from_inner(inner)
    )
}
pub fn with_msg_inner<T>(msg: impl Into<Cow<'static, str>>, inner: impl core::error::Error + 'static) -> Result<T> {
    Err(
        Error::new(msg, Some(inner), None)
    )
}
pub fn with_value<T>(value: Value) -> Result<T> {
    Err(
        Error::from_value(value)
    )
}
pub fn with_msg_value<T>(msg: impl Into<Cow<'static, str>>, value: Value) -> Result<T> {
    Err(
        Error::from_msg_value(msg, value)
    )
}
impl Error {
    pub fn new(msg: impl Into<Cow<'static, str>>, inner: Option<impl core::error::Error + 'static>, inner_value: Option<Value>) -> Self {
        let inner: Option<Box<dyn core::error::Error + 'static>> = 
            if let Some(inner) = inner {
                Some(Box::new(inner))
            } else {None};
        Self { msg: msg.into(), inner: inner, inner_value: inner_value.map(Box::new) }
    }

    pub fn from_inner(inner: impl core::error::Error + 'static) -> Self {
        Error { msg: "".into(), inner: Some(Box::new(inner)), inner_value: None }
    }
    pub fn from_msg(msg: impl Into<Cow<'static, str>>) -> Self {
        Error { msg: msg.into(), inner: None, inner_value: None }
    }
    pub fn from_value(value: Value) -> Self {
        Error { msg: "".into(), inner: None, inner_value: Some(Box::new(value)) }
    }
    pub fn from_msg_value(msg: impl Into<Cow<'static, str>>, value: Value) -> Self {
        Error { msg: msg.into(), inner: None, inner_value: Some(Box::new(value)) }
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

impl serde::de::Error for Error {
    fn custom<T>(msg:T) -> Self where T:Display {
        Error {
            msg: msg.to_string().into(),
            inner: None,
            inner_value: None,
        }
    }
}
impl From<rmpv::encode::Error> for Error {
    fn from(value: rmpv::encode::Error) -> Self {
        Error::from_inner(value)
    }
}
