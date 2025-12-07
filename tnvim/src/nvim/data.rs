use nvimapi::Color;
use serde::Deserialize;
use suffixes::CastIt;

use crate::app::App;

#[derive(Default)]
pub struct Data {
    pub color_set: ColorSet,
    pub hl_attrs: Vec<RgbAttrs>,
    pub grids: crate::HashMap<u16, Grid>,
    pub cursor: Cursor,
}

#[derive(Default, Debug)]
pub struct ColorSet {
    pub fg: Color,
    pub bg: Color,
    pub sp: Color,
}

#[derive(Deserialize, Debug, Default, Clone)]
#[serde(default)]
pub struct RgbAttrs {
    pub foreground: Option<Color>,
    pub background: Option<Color>,
    pub special: Option<Color>,
    pub reverse: bool,
    pub italic: bool,
    pub strikethrough: bool,
    pub underline: bool,
    pub undercurl: bool,
    pub underdouble: bool,
    pub underdotted: bool,
    pub underdashed: bool,
    pub altfont: bool,
    pub blend: u8,
    pub url: String,
}

#[derive(Debug, Default, Clone)]
pub struct Grid {
    pub width: u16,
    pub height: u16,
}
#[derive(Debug, Default, Clone)]
pub struct Cursor {
    pub row: u16,
    pub col: u16,
}

impl App {
    pub fn set_cursor(&self, col: u16, row: u16) {
        let mut data = self.nvimdata.borrow_mut();
        data.cursor.col = col;
        data.cursor.row = row;
        drop(data);
    }

}
