use core::cell::RefCell;

use crate::nvim;

#[derive(Default)]
pub struct App {
    pub nvimdata: RefCell::<nvim::Data>,
}
