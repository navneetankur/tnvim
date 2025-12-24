use core::fmt::Display;
use std::borrow::Cow;

#[derive(Debug)]
pub struct Error{
    msg: Cow<'static, str>,
    source: Option<Box<dyn core::error::Error>>,
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if !self.msg.is_empty() {
            f.write_str(&self.msg)?;
        }
        if let Some(e) = &self.source {
            write!(f, "cause by: {e}")?;
        }
        Ok(())
    }
}
impl core::error::Error for Error {
}
pub type Result<T> = core::result::Result<T, Error>;
impl From<std::io::Error> for Error {
    fn from(value: std::io::Error) -> Self {
        Self { source: Some(Box::new(value)), msg: "".into() }
    }
}
