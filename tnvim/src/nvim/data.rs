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
    pub size: Size,
}

#[derive(Default, Debug, Clone)]
pub struct Size {
    pub w: u16,
    pub h: u16,

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
#[derive(Default, Debug, Clone, Copy)]
pub enum GridType {
    #[default]
    Normal,
    Message,
    Main,
}
#[derive(Debug, Default, Clone)]
pub struct Grid {
    pub size: Size,
    pub pos: Position,
    pub grid_type: GridType,
}
#[derive(Debug, Default, Clone)]
pub struct Cursor {
    pub pos: Position,
}
#[derive(Debug, Default, Clone)]
pub struct Position {
    pub row: u16,
    pub col: u16,
}

impl App {
    pub fn set_cursor(&self, col: u16, row: u16) {
        let mut data = self.nvimdata.borrow_mut();
        data.cursor.pos.col = col;
        data.cursor.pos.row = row;
        drop(data);
    }

}
