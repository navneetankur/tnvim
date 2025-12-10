use crate::error;
use core::cell::RefCell;
use std::io::{Stdout, stdout};
use crossterm::{ExecutableCommand, QueueableCommand, style::Print, terminal::{self, LeaveAlternateScreen}};
pub use crossterm::event;
pub struct Terminal {
    stdout: RefCell<Stdout>,
}
type Ret<'t> = error::Result<&'t Terminal>;
impl Terminal {
    pub fn enter_alternate_screen(&self) -> Ret {
        self.stdout.borrow_mut().execute(crossterm::terminal::EnterAlternateScreen)?;
        Ok(self)
    }
    pub fn enable_raw_mode(&self) -> Ret {
        crossterm::terminal::enable_raw_mode()?;
        Ok(self)
    }

    pub fn set_title(&self, title: &str) -> error::Result<&Self> {
        self.stdout.borrow_mut().execute(crossterm::terminal::SetTitle(title))?;
        return Ok(self);
    }
    pub fn enable_mouse_events(&self) -> Ret {
        self.stdout.borrow_mut().execute(crossterm::event::EnableMouseCapture)?;
        Ok((self))
    }
    pub fn disable_mouse_events(&self) -> Ret {
        self.stdout.borrow_mut().execute(crossterm::event::DisableMouseCapture)?;
        Ok((self))
    }

    pub fn move_cursor(&self, col: u16, row: u16) -> error::Result<&Self> {
        self.stdout.borrow_mut().queue(crossterm::cursor::MoveTo(col, row))?;
        return Ok(self);
    }

    pub fn print(&self, text: &str) -> Ret<'_> {
        self.stdout.borrow_mut().queue(Print(text))?;
        Ok(self)
    }
    pub fn size(&self) -> error::Result<(u16, u16)> {
        let (w,h) = crossterm::terminal::size()?;
        Ok((w, h))
    }

    pub fn clear_row(&self) -> Ret<'_> {
        self.stdout.borrow_mut().queue(crossterm::terminal::Clear(terminal::ClearType::CurrentLine))?;
        Ok(self)
    }
    pub fn clear_screen(&self) -> Ret<'_> {
        self.stdout.borrow_mut().queue(crossterm::terminal::Clear(terminal::ClearType::All))?;
        Ok(self)
    }

    pub(crate) fn set_foreground_color(&self, fg: nvimapi::Color) -> Ret<'_> {
        self.stdout.borrow_mut().queue(crossterm::style::SetForegroundColor(crossterm::style::Color::Rgb { r: fg.r, g: fg.g, b: fg.b }))?;
        Ok(self)
    }
    pub(crate) fn set_background_color(&self, bg: nvimapi::Color) -> Ret<'_> {
        self.stdout.borrow_mut().queue(crossterm::style::SetBackgroundColor(crossterm::style::Color::Rgb { r: bg.r, g: bg.g, b: bg.b }))?;
        Ok(self)
    }
    pub(crate) fn set_colors(&self, bg: nvimapi::Color, fg:nvimapi::Color) -> Ret<'_> {
        self.stdout.borrow_mut().queue(crossterm::style::SetColors(crossterm::style::Colors {
            foreground: Some(crossterm::style::Color::Rgb { r: fg.r, g: fg.g, b: fg.b }),
            background: Some(crossterm::style::Color::Rgb { r: bg.r, g: bg.g, b: bg.b }),
        }))?;
        Ok(self)
    }
}
impl Default for Terminal {
    fn default() -> Self {
        Self {
            stdout: RefCell::new(stdout()),
        }
    }
}
pub fn init() {}
pub fn leave_alternate_screen() {
    stdout().execute(LeaveAlternateScreen).unwrap();
}
pub fn disable_raw_mode() {
    crossterm::terminal::disable_raw_mode().unwrap();
}
pub fn disable_mouse_events() -> std::result::Result<(), std::io::Error> {
    stdout().execute(crossterm::event::DisableMouseCapture).map(|_|())
}
