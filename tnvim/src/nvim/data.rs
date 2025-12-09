use nvimapi::Color;
use serde::Deserialize;
use suffixes::CastIt;
use veci1::VecI1;
use crate::app::App;

#[derive(Default)]
pub struct Data {
    pub color_set: ColorSet,
    pub hl_attrs: VecI1<RgbAttrs>,
    pub cursor: Cursor,
    pub size: Size,
    pub surface: grid::Grid<Cell>,
}

#[derive(Default, Debug, Clone, Copy)]
pub struct Cell {
    pub char_: char,
    pub hl: u16,
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
#[derive(Debug, Default, Clone, Copy)]
pub struct Position {
    pub col: u16,
    pub row: u16,
}

impl Position {
    pub fn new(col: u16, row: u16,) -> Self {
        Self {col, row}
    }
}
mod app {
use core::cell::RefCell;

use anyhow::anyhow;
use suffixes::WrappedResult;

use crate::{App, nvim::{Data, data::{Position, Size}}};

impl App {
    pub fn set_cursor(&self, col: u16, row: u16) {
        let mut data = self.nvimdata.borrow_mut();
        data.cursor.pos.col = col;
        data.cursor.pos.row = row;
        drop(data);
    }
}
}
