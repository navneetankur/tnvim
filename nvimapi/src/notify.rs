use rmpv::Value;
use serde::Deserialize;

use crate::namevalue::NameValue;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Notify {
    GridResize(GridResize),
    Unknown,
}
pub static GRID_RESIZE: &str = "grid_resize";
impl Notify {
    pub fn from_name_value(name: &str, value: Value) -> Self {
        let nv = NameValue::new(name, value);
        if let Ok(notify) = Self::deserialize(nv) {
            return notify;
        } else {
            return Self::Unknown;
        }
    }
}
pub fn from_value<'de, T: Deserialize<'de>>(value: Value) -> Result<T, rmpv::ext::Error> {
    T::deserialize(value)
}
#[derive(Deserialize, Debug)]
pub struct GridResize {
    pub grid: u16,
    pub width: u16,
    pub height: u16,
}
