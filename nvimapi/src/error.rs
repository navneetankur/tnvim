use core::fmt::Display;
use std::borrow::Cow;

#[derive(Debug)]
pub struct Error {
    msg: Cow<'static, str>,
}
impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.msg)
    }
}
impl core::error::Error for Error {}

pub fn from_cow<T>(msg: impl Into<Cow<'static, str>>) -> Result<T, Error> {
    Err(Error { msg: msg.into() })
}
impl serde::de::Error for Error {
    fn custom<T>(msg:T) -> Self where T:Display {
        Self { msg: msg.to_string().into() }
    }
}
