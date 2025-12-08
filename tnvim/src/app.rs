use core::cell::RefCell;

use terminal::Terminal;

use crate::nvim;

#[derive(Default)]
pub struct App {
    pub nvimdata: RefCell::<nvim::Data>,
    pub terminal: Terminal,
}
