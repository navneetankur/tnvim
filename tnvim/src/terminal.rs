use crate::error;
use core::cell::RefCell;
use std::io::{Write, stdout};
use crossterm::{ExecutableCommand, QueueableCommand, style::Print, terminal::{self, LeaveAlternateScreen}};
pub use crossterm::event;
pub struct Terminal {
    // out: RefCell<Stdout>,
    out: RefCell<Vec<u8>>,
}
type Ret<'t> = error::Result<&'t Terminal>;
impl Terminal {
    pub fn enter_alternate_screen(&self) -> Ret<'_> {
        stdout().execute(crossterm::terminal::EnterAlternateScreen)?;
        Ok(self)
    }
    pub fn enable_raw_mode(&self) -> Ret<'_> {
        crossterm::terminal::enable_raw_mode()?;
        Ok(self)
    }

    pub fn set_title(&self, title: &str) -> error::Result<&Self> {
        self.out.borrow_mut().queue(crossterm::terminal::SetTitle(title))?;
        Ok(self)
    }
    pub fn enable_mouse_events(&self) -> Ret<'_> {
        stdout().execute(crossterm::event::EnableMouseCapture)?;
        Ok(self )
    }
    pub fn enable_focus_events(&self) -> Ret<'_> {
        stdout().execute(crossterm::event::EnableFocusChange)?;
        Ok(self )
    }
    pub(crate) fn enable_bracketed_paste(&self) -> Ret<'_> {
        stdout().execute(crossterm::event::EnableBracketedPaste)?;
        Ok(self)
    }

    pub fn move_cursor(&self, col: u16, row: u16) -> error::Result<&Self> {
        self.out.borrow_mut().queue(crossterm::cursor::MoveTo(col, row))?;
        Ok(self)
    }

    pub fn print(&self, text: &str) -> Ret<'_> {
        self.out.borrow_mut().queue(Print(text))?;
        Ok(self)
    }
    pub fn size(&self) -> error::Result<(u16, u16)> {
        let (w,h) = crossterm::terminal::size()?;
        Ok((w, h))
    }
    pub fn clear_screen(&self) -> Ret<'_> {
        self.out.borrow_mut().queue(crossterm::terminal::Clear(terminal::ClearType::All))?;
        Ok(self)
    }
    pub(crate) fn set_colors(&self, bg: nvimapi::Color, fg:nvimapi::Color) -> Ret<'_> {
        self.out.borrow_mut().queue(crossterm::style::SetColors(crossterm::style::Colors {
            foreground: Some(crossterm::style::Color::Rgb { r: fg.r, g: fg.g, b: fg.b }),
            background: Some(crossterm::style::Color::Rgb { r: bg.r, g: bg.g, b: bg.b }),
        }))?;
        Ok(self)
    }
    pub(crate) fn set_cursor_shape(&self, cursor_shape: CursorShape) -> Ret<'_> {
        use crossterm::cursor::SetCursorStyle;
        let cursor_command = 
            match cursor_shape {
                CursorShape::Bar => SetCursorStyle::SteadyBar,
                CursorShape::Block => SetCursorStyle::SteadyBlock,
                CursorShape::UnderScore => SetCursorStyle::SteadyUnderScore,
                CursorShape::BarBlink => SetCursorStyle::BlinkingBar,
                CursorShape::BlockBlink => SetCursorStyle::BlinkingBlock,
                CursorShape::UnderScoreBlink => SetCursorStyle::BlinkingUnderScore,
            };
        self.out.borrow_mut().queue(cursor_command)?;
        Ok(self)
    }

    pub(crate) fn flush(&self) -> Ret<'_> {
        let mut out = self.out.borrow_mut();
        out.flush()?;
        let mut stdout = stdout();
        stdout.write_all(&out)?;
        stdout.flush()?;
        return Ok(self);
    }
}

#[derive(Clone, Copy, Debug)]
pub enum CursorShape {
    Bar,
    Block,
    UnderScore,
    BarBlink,
    BlockBlink,
    UnderScoreBlink,
}
impl Default for Terminal {
    fn default() -> Self {
        Self {
            out: RefCell::new(Vec::with_capacity(50 * 100)),
        }
    }
}
pub fn leave_alternate_screen() {
    stdout().execute(LeaveAlternateScreen).unwrap();
}
pub fn disable_raw_mode() {
    crossterm::terminal::disable_raw_mode().unwrap();
}
pub fn disable_bracketed_paste() -> std::result::Result<(), std::io::Error> {
    stdout().execute(crossterm::event::DisableBracketedPaste).map(|_|())
}
pub fn disable_mouse_events() -> std::result::Result<(), std::io::Error> {
    stdout().execute(crossterm::event::DisableMouseCapture).map(|_|())
}
pub fn disable_focus_events() -> Result<(), std::io::Error> {
    stdout().execute(crossterm::event::EnableFocusChange)?;
    Ok(())
}
