use nvimapi::{Nvimapi, uievent};
use crate::app::App;


pub(super) async fn do_set_scroll_region(this: &App, nvim: &impl Nvimapi, events: Vec<uievent::SetScrollRegion>) {
    log::info!("set_scroll_region");
    // for scroll in events {
    //     debug!("t: {}, b: {}, l: {}, r:{}", scroll.top, scroll.bot, scroll.left, scroll.right);
    // }
}
pub(super) async fn do_scroll(this: &App, nvim: &impl Nvimapi, events: Vec<uievent::Scroll>) {
    log::info!("scroll");
}
pub(super) async fn do_update_menu(this: &App, nvim: &impl Nvimapi, events: Vec<uievent::UpdateMenu>) {
    log::info!("update_menu");
}
pub(super) async fn do_busy_start(this: &App, nvim: &impl Nvimapi, events: Vec<uievent::BusyStart>) {
    log::info!("busy_start");
}
pub(super) async fn do_busy_stop(this: &App, nvim: &impl Nvimapi, events: Vec<uievent::BusyStop>) {
    log::info!("busy_stop");
}
pub(super) async fn do_mouse_off(this: &App, nvim: &impl Nvimapi, events: Vec<uievent::MouseOff>) {
    log::info!("mouse_off");
}
pub(super) async fn do_bell(this: &App, nvim: &impl Nvimapi, events: Vec<uievent::Bell>) {
    log::info!("bell");
}
pub(super) async fn do_visual_bell(this: &App, nvim: &impl Nvimapi, events: Vec<uievent::VisualBell>) {
    log::info!("visual_bell");
}
pub(super) async fn do_suspend(this: &App, nvim: &impl Nvimapi, events: Vec<uievent::Suspend>) {
    log::info!("suspend");
}
pub(super) async fn do_screenshot(this: &App, nvim: &impl Nvimapi, events: Vec<uievent::Screenshot>) {
    log::info!("screenshot");
}
pub(super) async fn do_update_fg(this: &App, nvim: &impl Nvimapi, events: Vec<uievent::UpdateFg>) {
    log::info!("update_fg");
}
pub(super) async fn do_update_bg(this: &App, nvim: &impl Nvimapi, events: Vec<uievent::UpdateBg>) {
    log::info!("update_bg");
}
pub(super) async fn do_update_sp(this: &App, nvim: &impl Nvimapi, events: Vec<uievent::UpdateSp>) {
    log::info!("update_sp");
}
pub(super) async fn do_resize(this: &App, nvim: &impl Nvimapi, events: Vec<uievent::Resize>) {
    log::info!("resize");
}
pub(super) async fn do_clear(this: &App, nvim: &impl Nvimapi, events: Vec<uievent::Clear>) {
    log::info!("clear");
}
pub(super) async fn do_eol_clear(this: &App, nvim: &impl Nvimapi, events: Vec<uievent::EolClear>) {
    log::info!("eol_clear");
}
pub(super) async fn do_cursor_goto(this: &App, nvim: &impl Nvimapi, events: Vec<uievent::CursorGoto>) {
    log::info!("cursor_goto");
}
pub(super) async fn do_highlight_set(this: &App, nvim: &impl Nvimapi, events: Vec<uievent::HighlightSet>) {
    log::info!("highlight_set");
}
pub(super) async fn do_put(this: &App, nvim: &impl Nvimapi, events: Vec<uievent::Put>) {
    log::info!("put");
}
pub(super) async fn do_grid_destroy(this: &App, nvim: &impl Nvimapi, events: Vec<uievent::GridDestroy>) {
    log::info!("grid_destroy");
}
pub(super) async fn do_win_float_pos(this: &App, nvim: &impl Nvimapi, events: Vec<uievent::WinFloatPos>) {
    log::info!("win_float_pos");
}
pub(super) async fn do_win_external_pos(this: &App, nvim: &impl Nvimapi, events: Vec<uievent::WinExternalPos>) {
    log::info!("win_external_pos");
}
pub(super) async fn do_win_hide(this: &App, nvim: &impl Nvimapi, events: Vec<uievent::WinHide>) {
    log::info!("win_hide");
}
pub(super) async fn do_win_close(this: &App, nvim: &impl Nvimapi, events: Vec<uievent::WinClose>) {
    log::info!("win_close");
}
pub(super) async fn do_win_viewport_margins(
    this: &App, nvim: &impl Nvimapi,
    events: Vec<uievent::WinViewportMargins>,
) {
    log::info!("win_viewport_margins");
}
pub(super) async fn do_win_extmark(this: &App, nvim: &impl Nvimapi, events: Vec<uievent::WinExtmark>) {
    log::info!("win_extmark");
}
pub(super) async fn do_popupmenu_show(this: &App, nvim: &impl Nvimapi, events: Vec<uievent::PopupmenuShow>) {
    log::info!("popupmenu_show");
}
pub(super) async fn do_popupmenu_hide(this: &App, nvim: &impl Nvimapi, events: Vec<uievent::PopupmenuHide>) {
    log::info!("popupmenu_hide");
}
pub(super) async fn do_popupmenu_select(this: &App, nvim: &impl Nvimapi, events: Vec<uievent::PopupmenuSelect>) {
    log::info!("popupmenu_select");
}
pub(super) async fn do_tabline_update(this: &App, nvim: &impl Nvimapi, events: Vec<uievent::TablineUpdate>) {
    log::info!("tabline_update");
}
pub(super) async fn do_cmdline_show(this: &App, nvim: &impl Nvimapi, events: Vec<uievent::CmdlineShow>) {
    log::info!("cmdline_show");
}
pub(super) async fn do_cmdline_pos(this: &App, nvim: &impl Nvimapi, events: Vec<uievent::CmdlinePos>) {
    log::info!("cmdline_pos");
}
pub(super) async fn do_cmdline_special_char(
    this: &App, nvim: &impl Nvimapi,
    events: Vec<uievent::CmdlineSpecialChar>,
) {
    log::info!("cmdline_special_char");
}
pub(super) async fn do_cmdline_hide(this: &App, nvim: &impl Nvimapi, events: Vec<uievent::CmdlineHide>) {
    log::info!("cmdline_hide");
}
pub(super) async fn do_cmdline_block_show(this: &App, nvim: &impl Nvimapi, events: Vec<uievent::CmdlineBlockShow>) {
    log::info!("cmdline_block_show");
}
pub(super) async fn do_cmdline_block_append(
    this: &App, nvim: &impl Nvimapi,
    events: Vec<uievent::CmdlineBlockAppend>,
) {
    log::info!("cmdline_block_append");
}
pub(super) async fn do_cmdline_block_hide(this: &App, nvim: &impl Nvimapi, events: Vec<uievent::CmdlineBlockHide>) {
    log::info!("cmdline_block_hide");
}
pub(super) async fn do_wildmenu_show(this: &App, nvim: &impl Nvimapi, events: Vec<uievent::WildmenuShow>) {
    log::info!("wildmenu_show");
}
pub(super) async fn do_wildmenu_select(this: &App, nvim: &impl Nvimapi, events: Vec<uievent::WildmenuSelect>) {
    log::info!("wildmenu_select");
}
pub(super) async fn do_wildmenu_hide(this: &App, nvim: &impl Nvimapi, events: Vec<uievent::WildmenuHide>) {
    log::info!("wildmenu_hide");
}
pub(super) async fn do_msg_show(this: &App, nvim: &impl Nvimapi, events: Vec<uievent::MsgShow>) {
    log::info!("msg_show");
}
pub(super) async fn do_msg_clear(this: &App, nvim: &impl Nvimapi, events: Vec<uievent::MsgClear>) {
    log::info!("msg_clear");
}
pub(super) async fn do_msg_showcmd(this: &App, nvim: &impl Nvimapi, events: Vec<uievent::MsgShowcmd>) {
    log::info!("msg_showcmd");
}
pub(super) async fn do_msg_showmode(this: &App, nvim: &impl Nvimapi, events: Vec<uievent::MsgShowmode>) {
    log::info!("msg_showmode");
}
pub(super) async fn do_msg_ruler(this: &App, nvim: &impl Nvimapi, events: Vec<uievent::MsgRuler>) {
    log::info!("msg_ruler");
}
pub(super) async fn do_msg_history_show(this: &App, nvim: &impl Nvimapi, events: Vec<uievent::MsgHistoryShow>) {
    log::info!("msg_history_show");
}
pub(super) async fn do_msg_history_clear(this: &App, nvim: &impl Nvimapi, events: Vec<uievent::MsgHistoryClear>) {
    log::info!("msg_history_clear");
}
pub(super) async fn do_error_exit(this: &App, nvim: &impl Nvimapi, events: Vec<uievent::ErrorExit>) {
    log::info!("error_exit");
}
