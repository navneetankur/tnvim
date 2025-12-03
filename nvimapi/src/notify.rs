use rmpv::Value;
use serde::Deserialize;

use crate::valuedeserilizer::ValueDeserilizer;

#[derive(Debug)]
pub enum Notify {
    GridResize(GridResize),
    Unknown(String, Value),
}
pub static GRID_RESIZE: &str = "grid_resize";
impl Notify {
    pub fn from_name_value(name: &str, value: Value) -> Self {
        match name {
            "grid_resize" => {
                let rvi = from_value(value).unwrap();
                return Notify::GridResize(rvi);
            },
            _ => {
                Self::Unknown(name.to_string(), value)
            },
        }
    }
}

#[derive(Deserialize, Debug)]
pub struct GridResize {
    pub grid: u16,
    pub width: u16,
    pub height: u16,
}
pub fn from_value<'de, T: Deserialize<'de>>(value: Value) -> Result<T, crate::error::Error> {
    T::deserialize(ValueDeserilizer(value))
}
