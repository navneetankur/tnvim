pub mod error;
use core::cell::RefCell;
use std::io::{Stdout, stdout};
pub use crossterm::event;
use crossterm::{ExecutableCommand, QueueableCommand, style::Print, terminal::{self, LeaveAlternateScreen}};
pub struct Terminal {
    stdout: RefCell<Stdout>,
}
type Ret<'t> = error::Result<&'t Terminal>;
impl Terminal {
    pub fn enter_alternate_screen(&self) {
        self.stdout.borrow_mut().execute(crossterm::terminal::EnterAlternateScreen).unwrap();
    }
    pub fn enable_raw_mode(&self) {
        crossterm::terminal::enable_raw_mode().unwrap();
    }

    pub fn set_title(&self, title: &str) -> error::Result<&Self> {
        self.stdout.borrow_mut().execute(crossterm::terminal::SetTitle(title))?;
        return Ok(self);
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
