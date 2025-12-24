use log::debug;
use nvimapi::{Color, uievent::Tabpage};
use serde::Deserialize;
use suffixes::CastIt;
use veci1::VecI1;
use crate::{app::App, terminal::{CursorShape, Terminal}};

#[derive(Default)]
pub struct Data {
    pub color_set: ColorSet,
    pub hl_attrs: VecI1<RgbAttrs>,
    pub cursor: Cursor,
    pub nvim_size: Size,
    pub ui_size: Size,
    pub surface: grid::Grid<Cell>,
    pub current_hl_id: u16,
    pub mode_cursors: Vec<CursorShape>,
    pub my_tab: Option<Tabpage>,
}

#[derive(Debug, Clone, Copy)]
pub struct Cell {
    pub char_: char,
    pub hl: u16,
}
impl Default for Cell {
    fn default() -> Self {
        Self { char_: ' ', hl: 0 }
    }
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

impl Data {
    pub fn apply_hl_id_forced(&mut self, hl_id: u16, term: &Terminal) {
        self.current_hl_id = hl_id;
        let dbg = self.color_set.bg;
        let dfg = self.color_set.fg;
        let dsp = self.color_set.sp;
        if hl_id == 0 {
            term.set_colors(dbg, dfg).unwrap();
            return;
        }

        let rgb_attr = &self.hl_attrs[hl_id];
        
        let bg = rgb_attr.background.unwrap_or(dbg);
        let fg = rgb_attr.foreground.unwrap_or(dfg);
        let (bg, fg) = 
            if rgb_attr.reverse {
                (fg, bg)
            } else {
                (bg, fg)
            };
        // debug!("fg: {fg:?}, bg: {bg:?}");
        let sp = rgb_attr.special.unwrap_or(dsp);
        term.set_colors(bg, fg,).unwrap();
    }
    pub fn apply_hl_id(&mut self, hl_id: u16, term: &Terminal) {
        if self.current_hl_id == hl_id {return}
        self.apply_hl_id_forced(hl_id, term);
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
