use std::io::stdout;
use crossterm::terminal;
use crossterm::ExecutableCommand;
use nvimapi::{Nvimapi, uievent};
use rmpv::Value;
use crate::app::App;

pub(super) async fn do_mouse_on(this: &App, nvim: &impl Nvimapi, mouse_ons: Vec<uievent::MouseOn>) {
    // mouse is on?. I think.
}

pub(super) async fn do_mode_change(this: &App, nvim: &impl Nvimapi, mode_changes: Vec<uievent::ModeChange>) {
    // mode (normal, insert) has changed.
}

pub(super) async fn do_mode_info_set(this: &App, nvim: &impl Nvimapi, mode_info_sets: Vec<uievent::ModeInfoSet>) {
    // how should cursor look line in a particular mode.
}
pub(super) async fn do_option_set(this: &App, nvim: &impl Nvimapi, events: Vec<uievent::OptionSet>) {
    // here vim tells about some option which are set color, mouse and so on.
    // don't see any use for now, as i will be using terminal's font anyways.
}
pub(super) async fn do_set_icon(this: &App, nvim: &impl Nvimapi, events: Vec<uievent::SetIcon>) {
    //minimized window title.
}
// to consider
pub(super) async fn do_chdir(this: &App, nvim: &impl Nvimapi, events: Vec<uievent::Chdir>) {
    //cwd changed for vim. Do i need to change too.
    //Doesn't seem necessary for now.
}
pub(super) async fn do_hl_group_set(this: &App, nvim: &impl Nvimapi, events: Vec<uievent::HlGroupSet>) {
    //gives a name to hl_id.
    //I don't think this name is used by nvim. but is only four plugins. Need to see more.
}
pub(super) async fn do_win_viewport(this: &App, nvim: &impl Nvimapi, events: Vec<uievent::WinViewport>) {
    //supposed to help with smooth scrolling.
}
