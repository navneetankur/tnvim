mod data;
pub use data::Data;
use log::{debug, warn};
use nvimapi::{Handler, Notification, Nvimapi, Pairs, Request, UiEvent, UiOptions, uievent, NvimapiNr};
use crate::app::App;
pub const MAIN_GRID: u8 = 1;

impl Handler for App {
    async fn notify(&self, nvim: &impl Nvimapi, notification: Notification) {
        log::trace!("notify");
        let redraw = notification.into_redraw();
        for event in redraw {
            handle_uievent(self, nvim, event).await;
        }
    }

    async fn request(&self, nvim: &impl Nvimapi, request: Box<Request>) {
        debug!("request");
    }

    async fn init(&self, nvim: &impl Nvimapi) {
        debug!("init");
        let (w,h) = self.terminal.size().unwrap();
        nvim.noret().ui_attach(
        // nvim.ui_attach(
            w.into(),
           h.into(),
           Pairs::new().with_iter([
               (UiOptions::Rgb, true),
               (UiOptions::ExtLinegrid, true),
               (UiOptions::ExtMultigrid, true),
               // (UiOptions::ExtCmdline, true),
               // (UiOptions::ExtTabline, true),
               // not sure what below ones do. Let's implement these for now,
               // and see if it's enough.
               // (UiOptions::ExtHlstate, true),
               // (UiOptions::ExtWildmenu, true),
               // (UiOptions::ExtMessages, true),
               // (UiOptions::ExtTermcolors, true),
               // (UiOptions::ExtPopupmenu, true),
           ])
        ).unwrap();
        // ).await.unwrap();
    }
}

#[allow(unused_variables)]
mod unencounteredevents;
#[allow(unused_variables)]
mod encounteredevents;
mod implementedevents;
async fn handle_uievent(this: &App, nvim: &impl Nvimapi, event: UiEvent) {
    use unencounteredevents::*;
    use encounteredevents::*;
    use implementedevents::*;
    match event {
        UiEvent::ModeInfoSet(events) => do_mode_info_set(this, nvim, events).await,
        UiEvent::UpdateMenu(events) => do_update_menu(this, nvim, events).await,
        UiEvent::BusyStart(events) => do_busy_start(this, nvim, events).await,
        UiEvent::BusyStop(events) => do_busy_stop(this, nvim, events).await,
        UiEvent::MouseOn(events) => do_mouse_on(this, nvim, events).await,
        UiEvent::MouseOff(events) => do_mouse_off(this, nvim, events).await,
        UiEvent::ModeChange(events) => do_mode_change(this, nvim, events).await,
        UiEvent::Bell(events) => do_bell(this, nvim, events).await,
        UiEvent::VisualBell(events) => do_visual_bell(this, nvim, events).await,
        UiEvent::Flush(events) => do_flush(this, nvim, events).await,
        UiEvent::Suspend(events) => do_suspend(this, nvim, events).await,
        UiEvent::SetTitle(events) => do_set_title(this, nvim, events).await,
        UiEvent::SetIcon(events) => do_set_icon(this, nvim, events).await,
        UiEvent::Screenshot(events) => do_screenshot(this, nvim, events).await,
        UiEvent::OptionSet(events) => do_option_set(this, nvim, events).await,
        UiEvent::Chdir(events) => do_chdir(this, nvim, events).await,
        UiEvent::UpdateFg(events) => do_update_fg(this, nvim, events).await,
        UiEvent::UpdateBg(events) => do_update_bg(this, nvim, events).await,
        UiEvent::UpdateSp(events) => do_update_sp(this, nvim, events).await,
        UiEvent::Resize(events) => do_resize(this, nvim, events).await,
        UiEvent::Clear(events) => do_clear(this, nvim, events).await,
        UiEvent::EolClear(events) => do_eol_clear(this, nvim, events).await,
        UiEvent::CursorGoto(events) => do_cursor_goto(this, nvim, events).await,
        UiEvent::HighlightSet(events) => do_highlight_set(this, nvim, events).await,
        UiEvent::Put(events) => do_put(this, nvim, events).await,
        UiEvent::SetScrollRegion(events) => do_set_scroll_region(this, nvim, events).await,
        UiEvent::Scroll(events) => do_scroll(this, nvim, events).await,
        UiEvent::DefaultColorsSet(events) => do_default_colors_set(this, nvim, events).await,
        UiEvent::HlAttrDefine(events) => do_hl_attr_define(this, nvim, events).await,
        UiEvent::HlGroupSet(events) => do_hl_group_set(this, nvim, events).await,
        UiEvent::GridResize(events) => do_grid_resize(this, nvim, events).await,
        UiEvent::GridClear(events) => do_grid_clear(this, nvim, events).await,
        UiEvent::GridCursorGoto(events) => do_grid_cursor_goto(this, nvim, events).await,
        UiEvent::GridLine(events) => do_grid_line(this, nvim, events).await,
        UiEvent::GridScroll(events) => do_grid_scroll(this, nvim, events).await,
        UiEvent::GridDestroy(events) => do_grid_destroy(this, nvim, events).await,
        UiEvent::WinPos(events) => do_win_pos(this, nvim, events).await,
        UiEvent::WinFloatPos(events) => do_win_float_pos(this, nvim, events).await,
        UiEvent::WinExternalPos(events) => do_win_external_pos(this, nvim, events).await,
        UiEvent::WinHide(events) => do_win_hide(this, nvim, events).await,
        UiEvent::WinClose(events) => do_win_close(this, nvim, events).await,
        UiEvent::MsgSetPos(events) => do_msg_set_pos(this, nvim, events).await,
        UiEvent::WinViewport(events) => do_win_viewport(this, nvim, events).await,
        UiEvent::WinViewportMargins(events) => do_win_viewport_margins(this, nvim, events).await,
        UiEvent::WinExtmark(events) => do_win_extmark(this, nvim, events).await,
        UiEvent::PopupmenuShow(events) => do_popupmenu_show(this, nvim, events).await,
        UiEvent::PopupmenuHide(events) => do_popupmenu_hide(this, nvim, events).await,
        UiEvent::PopupmenuSelect(events) => do_popupmenu_select(this, nvim, events).await,
        UiEvent::TablineUpdate(events) => do_tabline_update(this, nvim, events).await,
        UiEvent::CmdlineShow(events) => do_cmdline_show(this, nvim, events).await,
        UiEvent::CmdlinePos(events) => do_cmdline_pos(this, nvim, events).await,
        UiEvent::CmdlineSpecialChar(events) => do_cmdline_special_char(this, nvim, events).await,
        UiEvent::CmdlineHide(events) => do_cmdline_hide(this, nvim, events).await,
        UiEvent::CmdlineBlockShow(events) => do_cmdline_block_show(this, nvim, events).await,
        UiEvent::CmdlineBlockAppend(events) => do_cmdline_block_append(this, nvim, events).await,
        UiEvent::CmdlineBlockHide(events) => do_cmdline_block_hide(this, nvim, events).await,
        UiEvent::WildmenuShow(events) => do_wildmenu_show(this, nvim, events).await,
        UiEvent::WildmenuSelect(events) => do_wildmenu_select(this, nvim, events).await,
        UiEvent::WildmenuHide(events) => do_wildmenu_hide(this, nvim, events).await,
        UiEvent::MsgShow(events) => do_msg_show(this, nvim, events).await,
        UiEvent::MsgClear(events) => do_msg_clear(this, nvim, events).await,
        UiEvent::MsgShowcmd(events) => do_msg_showcmd(this, nvim, events).await,
        UiEvent::MsgShowmode(events) => do_msg_showmode(this, nvim, events).await,
        UiEvent::MsgRuler(events) => do_msg_ruler(this, nvim, events).await,
        UiEvent::MsgHistoryShow(events) => do_msg_history_show(this, nvim, events).await,
        UiEvent::MsgHistoryClear(events) => do_msg_history_clear(this, nvim, events).await,
        UiEvent::ErrorExit(events) => do_error_exit(this, nvim, events).await,
        UiEvent::Unknown(unknown) => do_unknown(this, unknown).await,
    }
}

async fn do_unknown(_: &App, unknown: Box<(String, rmpv::Value)>) {
    warn!("unknown uievent {}", &unknown.0);
}
