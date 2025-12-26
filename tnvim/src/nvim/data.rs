use nvimapi::{Color, uievent::Tabpage};
use serde::Deserialize;
use suffixes::CastIt;
use veci1::VecI1;
use crate::terminal::{CursorShape, Terminal};

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
    pub attached: bool,
}
// saving char insted of string here makes display of multicodepoint input wrong.
#[derive(Debug, Clone)]
pub struct Cell {
    pub char_: Grapheme,
    pub hl: u16,
}
#[derive(Clone, Copy)]
pub struct Char {
    data: [u8;7],
    length: u8,
}
// point of this struct is to keep normal chars in stack, without heap allocation.
#[derive(Clone)]
pub enum Grapheme {
    Char(Char),
    Str(std::rc::Rc<str>),
}
impl Default for Cell {
    fn default() -> Self {
        Self { char_: Grapheme::space(), hl: 0 }
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
#[allow(dead_code)]
#[derive(Default, Debug, Clone, Copy)]
pub enum GridType {
    #[default]
    Normal,
    Message,
    Main,
}
#[allow(dead_code)]
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
    #[allow(dead_code)]
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
        let _sp = rgb_attr.special.unwrap_or(dsp);
        term.set_colors(bg, fg,).unwrap();
    }
    pub fn apply_hl_id(&mut self, hl_id: u16, term: &Terminal) {
        if self.current_hl_id == hl_id {return}
        self.apply_hl_id_forced(hl_id, term);
    }
}
impl AsRef<str> for Grapheme {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl core::fmt::Debug for Grapheme {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let as_str = 
            match self {
                Self::Char(arg0) => arg0.as_str(),
                Self::Str(arg0) => &arg0,
            };
        return core::fmt::Debug::fmt(as_str, f);
    }
}
impl core::fmt::Display for Grapheme {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let as_str = 
            match self {
                Self::Char(arg0) => arg0.as_str(),
                Self::Str(arg0) => &arg0,
            };
        return core::fmt::Display::fmt(as_str, f);
    }
}
impl Grapheme {
    pub fn as_str(&self) -> &str {
        match self {
            Grapheme::Char(char_) => char_.as_str(),
            Grapheme::Str(rcstr) => &rcstr,
        }
    }
    pub const fn space() -> Self {
        let mut data = [0;7];
        data[0] = b' ';
        return Self::Char(Char { data, length: 1 });
    }
}
impl From<&str> for Grapheme {
    fn from(value: &str) -> Self {
        if let Some(char_) = Char::from_str(value) {
            Self::Char(char_)
        } else {
            Self::Str(value.into())
        }
    }
}
impl Char {
    pub fn from_str(value: &str) -> Option<Self> {
        let bytes = value.as_bytes();
        if bytes.len() <= 7 {
            let mut data = [0;7];
            data[..bytes.len()].copy_from_slice(bytes);
            return Some(Self {
                data,
                length: bytes.len().u8(),
            });
        } else {
            return None;
        }
    }
    pub fn as_str(&self) -> &str {
        let data = &self.data[..self.length.u()];
        #[cfg(debug_assertions)]
        return str::from_utf8(data).unwrap();
        #[cfg(not(debug_assertions))]
        return unsafe { str::from_utf8_unchecked(data) };
    }
}
impl Cell {
    pub fn new(char_: &str, hl: u16) -> Self {
        Self {
            char_: Grapheme::from(char_),
            hl,
        }
    }
}



mod app {


use crate::App;
impl App {
    pub fn set_cursor(&self, col: u16, row: u16) {
        let mut data = self.nvimdata.borrow_mut();
        data.cursor.pos.col = col;
        data.cursor.pos.row = row;
        drop(data);
    }
}
}
