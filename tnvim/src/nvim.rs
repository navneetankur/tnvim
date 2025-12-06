use log::debug;
use nvimapi::{Handler, Notification, Nvimapi, Request, UiEvent, uievent};
use crate::app::App;

impl Handler for App {
    async fn notify(&self, nvim: &impl Nvimapi, notification: Notification) {
        debug!("notify");
        let redraw = notification.into_redraw();
        for event in redraw {
            handle_uievent(nvim, event);
        }
    }

    async fn request(&self, nvim: &impl Nvimapi, request: Box<Request>) {
        debug!("request");
    }

    async fn init(&self, nvim: &impl Nvimapi) {
        debug!("init");
        let (w,h) = crossterm::terminal::size().unwrap();
        nvim.ui_attach(w.into(), h.into(), [0;0]).await.unwrap();
    }
}

fn handle_uievent(nvim: &impl Nvimapi, event: UiEvent) {
    match event {
        UiEvent::ModeInfoSet(mode_info_sets) => do_mode_info_sets(nvim, mode_info_sets),
        UiEvent::UpdateMenu(update_menus) => do_update_menus(nvim, update_menus),
        UiEvent::BusyStart(busy_starts) => do_busy_starts(nvim, busy_starts),
        UiEvent::BusyStop(busy_stops) => do_busy_stops(nvim, busy_stops),
        UiEvent::MouseOn(mouse_ons) => do_mouse_ons(nvim, mouse_ons),
        UiEvent::MouseOff(mouse_offs) => do_mouse_offs(nvim, mouse_offs),
        UiEvent::ModeChange(mode_changes) => do_mode_changes(nvim, mode_changes),
        UiEvent::Bell(bells) => do_bells(nvim, bells),
        UiEvent::VisualBell(visual_bells) => do_visual_bells(nvim, visual_bells),
        UiEvent::Flush(flushs) => do_flushs(nvim, flushs),
        UiEvent::Suspend(suspends) => do_suspends(nvim, suspends),
        UiEvent::SetTitle(set_titles) => do_set_titles(nvim, set_titles),
        UiEvent::SetIcon(set_icons) => do_set_icons(nvim, set_icons),
        UiEvent::Screenshot(screenshots) => do_screenshots(nvim, screenshots),
        UiEvent::OptionSet(option_sets) => do_option_sets(nvim, option_sets),
        UiEvent::Chdir(chdirs) => do_chdirs(nvim, chdirs),
        UiEvent::UpdateFg(update_fgs) => do_update_fgs(nvim, update_fgs),
        UiEvent::UpdateBg(update_bgs) => do_update_bgs(nvim, update_bgs),
        UiEvent::UpdateSp(update_sps) => do_update_sps(nvim, update_sps),
        UiEvent::Resize(resizes) => do_resizes(nvim, resizes),
        UiEvent::Clear(clears) => do_clears(nvim, clears),
        UiEvent::EolClear(eol_clears) => do_eol_clears(nvim, eol_clears),
        UiEvent::CursorGoto(cursor_gotos) => do_cursor_gotos(nvim, cursor_gotos),
        UiEvent::HighlightSet(highlight_sets) => do_highlight_sets(nvim, highlight_sets),
        UiEvent::Put(puts) => do_puts(nvim, puts),
        UiEvent::SetScrollRegion(set_scroll_regions) => do_set_scroll_regions(nvim, set_scroll_regions),
        UiEvent::Scroll(scrolls) => do_scrolls(nvim, scrolls),
        UiEvent::DefaultColorsSet(default_colors_sets) => do_default_colors_sets(nvim, default_colors_sets),
        UiEvent::HlAttrDefine(hl_attr_defines) => do_hl_attr_defines(nvim, hl_attr_defines),
        UiEvent::HlGroupSet(hl_group_sets) => do_hl_group_sets(nvim, hl_group_sets),
        UiEvent::GridResize(grid_resizes) => do_grid_resizes(nvim, grid_resizes),
        UiEvent::GridClear(grid_clears) => do_grid_clears(nvim, grid_clears),
        UiEvent::GridCursorGoto(grid_cursor_gotos) => do_grid_cursor_gotos(nvim, grid_cursor_gotos),
        UiEvent::GridLine(grid_lines) => do_grid_lines(nvim, grid_lines),
        UiEvent::GridScroll(grid_scrolls) => do_grid_scrolls(nvim, grid_scrolls),
        UiEvent::GridDestroy(items) => do_grid_destroy(nvim, items),
        UiEvent::WinPos(items) => do_win_pos(nvim, items),
        UiEvent::WinFloatPos(items) => do_win_float_pos(nvim, items),
        UiEvent::WinExternalPos(items) => do_win_external_pos(nvim, items),
        UiEvent::WinHide(win_hides) => do_win_hides(nvim, win_hides),
        UiEvent::WinClose(win_closes) => do_win_closes(nvim, win_closes),
        UiEvent::MsgSetPos(items) => do_msg_get_pos(nvim, items),
        UiEvent::WinViewport(win_viewports) => do_win_viewports(nvim, win_viewports),
        UiEvent::WinViewportMargins(items) => do_win_viewport_margins(nvim, items),
        UiEvent::WinExtmark(win_extmarks) => do_win_extmarks(nvim, win_extmarks),
        UiEvent::PopupmenuShow(popupmenu_shows) => do_popupmenu_shows(nvim, popupmenu_shows),
        UiEvent::PopupmenuHide(popupmenu_hides) => do_popupmenu_hides(nvim, popupmenu_hides),
        UiEvent::PopupmenuSelect(popupmenu_selects) => do_popupmenu_selects(nvim, popupmenu_selects),
        UiEvent::TablineUpdate(tabline_updates) => do_tabline_updates(nvim, tabline_updates),
        UiEvent::CmdlineShow(cmdline_shows) => do_cmdline_shows(nvim, cmdline_shows),
        UiEvent::CmdlinePos(items) => do_cmdline_pos(nvim, items),
        UiEvent::CmdlineSpecialChar(cmdline_special_chars) => do_cmdline_special_chars(nvim, cmdline_special_chars),
        UiEvent::CmdlineHide(cmdline_hides) => do_cmdline_hides(nvim, cmdline_hides),
        UiEvent::CmdlineBlockShow(cmdline_block_shows) => do_cmdline_block_shows(nvim, cmdline_block_shows),
        UiEvent::CmdlineBlockAppend(cmdline_block_appends) => do_cmdline_block_appends(nvim, cmdline_block_appends),
        UiEvent::CmdlineBlockHide(cmdline_block_hides) => do_cmdline_block_hides(nvim, cmdline_block_hides),
        UiEvent::WildmenuShow(wildmenu_shows) => do_wildmenu_shows(nvim, wildmenu_shows),
        UiEvent::WildmenuSelect(wildmenu_selects) => do_wildmenu_selects(nvim, wildmenu_selects),
        UiEvent::WildmenuHide(wildmenu_hides) => do_wildmenu_hides(nvim, wildmenu_hides),
        UiEvent::MsgShow(msg_shows) => do_msg_shows(nvim, msg_shows),
        UiEvent::MsgClear(msg_clears) => do_msg_clears(nvim, msg_clears),
        UiEvent::MsgShowcmd(msg_showcmds) => do_msg_showcmds(nvim, msg_showcmds),
        UiEvent::MsgShowmode(msg_showmodes) => do_msg_showmodes(nvim, msg_showmodes),
        UiEvent::MsgRuler(msg_rulers) => do_msg_rulers(nvim, msg_rulers),
        UiEvent::MsgHistoryShow(msg_history_shows) => do_msg_history_shows(nvim, msg_history_shows),
        UiEvent::MsgHistoryClear(msg_history_clears) => do_msg_history_clears(nvim, msg_history_clears),
        UiEvent::ErrorExit(error_exits) => do_error_exits(nvim, error_exits),
        UiEvent::Unknown(unknown) => do_unknown(nvim, unknown),
    }
}

fn do_unknown(nvim: &impl Nvimapi, unknown: Box<(String, rmpv::Value)>) {
    todo!()
}

fn do_error_exits(nvim: &impl Nvimapi, error_exits: Vec<uievent::ErrorExit>) {
    todo!()
}

fn do_msg_history_clears(nvim: &impl Nvimapi, msg_history_clears: Vec<uievent::MsgHistoryClear>) {
    todo!()
}

fn do_msg_history_shows(nvim: &impl Nvimapi, msg_history_shows: Vec<uievent::MsgHistoryShow>) {
    todo!()
}

fn do_msg_rulers(nvim: &impl Nvimapi, msg_rulers: Vec<uievent::MsgRuler>) {
    todo!()
}

fn do_msg_showmodes(nvim: &impl Nvimapi, msg_showmodes: Vec<uievent::MsgShowmode>) {
    todo!()
}

fn do_msg_showcmds(nvim: &impl Nvimapi, msg_showcmds: Vec<uievent::MsgShowcmd>) {
    todo!()
}

fn do_msg_clears(nvim: &impl Nvimapi, msg_clears: Vec<uievent::MsgClear>) {
    todo!()
}

fn do_msg_shows(nvim: &impl Nvimapi, msg_shows: Vec<uievent::MsgShow>) {
    todo!()
}

fn do_wildmenu_hides(nvim: &impl Nvimapi, wildmenu_hides: Vec<uievent::WildmenuHide>) {
    todo!()
}

fn do_wildmenu_selects(nvim: &impl Nvimapi, wildmenu_selects: Vec<uievent::WildmenuSelect>) {
    todo!()
}

fn do_wildmenu_shows(nvim: &impl Nvimapi, wildmenu_shows: Vec<uievent::WildmenuShow>) {
    todo!()
}

fn do_cmdline_block_hides(nvim: &impl Nvimapi, cmdline_block_hides: Vec<uievent::CmdlineBlockHide>) {
    todo!()
}

fn do_cmdline_block_appends(nvim: &impl Nvimapi, cmdline_block_appends: Vec<uievent::CmdlineBlockAppend>) {
    todo!()
}

fn do_cmdline_block_shows(nvim: &impl Nvimapi, cmdline_block_shows: Vec<uievent::CmdlineBlockShow>) {
    todo!()
}

fn do_cmdline_hides(nvim: &impl Nvimapi, cmdline_hides: Vec<uievent::CmdlineHide>) {
    todo!()
}

fn do_cmdline_special_chars(nvim: &impl Nvimapi, cmdline_special_chars: Vec<uievent::CmdlineSpecialChar>) {
    todo!()
}

fn do_cmdline_pos(nvim: &impl Nvimapi, items: Vec<uievent::CmdlinePos>) {
    todo!()
}

fn do_cmdline_shows(nvim: &impl Nvimapi, cmdline_shows: Vec<uievent::CmdlineShow>) {
    todo!()
}

fn do_tabline_updates(nvim: &impl Nvimapi, tabline_updates: Vec<uievent::TablineUpdate>) {
    todo!()
}

fn do_popupmenu_selects(nvim: &impl Nvimapi, popupmenu_selects: Vec<uievent::PopupmenuSelect>) {
    todo!()
}

fn do_popupmenu_hides(nvim: &impl Nvimapi, popupmenu_hides: Vec<uievent::PopupmenuHide>) {
    todo!()
}

fn do_popupmenu_shows(nvim: &impl Nvimapi, popupmenu_shows: Vec<uievent::PopupmenuShow>) {
    todo!()
}

fn do_win_extmarks(nvim: &impl Nvimapi, win_extmarks: Vec<uievent::WinExtmark>) {
    todo!()
}

fn do_win_viewport_margins(nvim: &impl Nvimapi, items: Vec<uievent::WinViewportMargins>) {
    todo!()
}

fn do_win_viewports(nvim: &impl Nvimapi, win_viewports: Vec<uievent::WinViewport>) {
    todo!()
}

fn do_msg_get_pos(nvim: &impl Nvimapi, items: Vec<uievent::MsgSetPos>) {
    todo!()
}

fn do_win_closes(nvim: &impl Nvimapi, win_closes: Vec<uievent::WinClose>) {
    todo!()
}

fn do_win_hides(nvim: &impl Nvimapi, win_hides: Vec<uievent::WinHide>) {
    todo!()
}

fn do_win_external_pos(nvim: &impl Nvimapi, items: Vec<uievent::WinExternalPos>) {
    todo!()
}

fn do_win_float_pos(nvim: &impl Nvimapi, items: Vec<uievent::WinFloatPos>) {
    todo!()
}

fn do_win_pos(nvim: &impl Nvimapi, items: Vec<uievent::WinPos>) {
    todo!()
}

fn do_grid_destroy(nvim: &impl Nvimapi, items: Vec<uievent::GridDestroy>) {
    todo!()
}

fn do_grid_scrolls(nvim: &impl Nvimapi, grid_scrolls: Vec<uievent::GridScroll>) {
    todo!()
}

fn do_grid_lines(nvim: &impl Nvimapi, grid_lines: Vec<uievent::GridLine>) {
    todo!()
}

fn do_grid_cursor_gotos(nvim: &impl Nvimapi, grid_cursor_gotos: Vec<uievent::GridCursorGoto>) {
    todo!()
}

fn do_grid_clears(nvim: &impl Nvimapi, grid_clears: Vec<uievent::GridClear>) {
    todo!()
}

fn do_grid_resizes(nvim: &impl Nvimapi, grid_resizes: Vec<uievent::GridResize>) {
    todo!()
}

fn do_hl_group_sets(nvim: &impl Nvimapi, hl_group_sets: Vec<uievent::HlGroupSet>) {
    todo!()
}

fn do_hl_attr_defines(nvim: &impl Nvimapi, hl_attr_defines: Vec<uievent::HlAttrDefine>) {
    todo!()
}

fn do_default_colors_sets(nvim: &impl Nvimapi, default_colors_sets: Vec<uievent::DefaultColorsSet>) {
    todo!()
}

fn do_scrolls(nvim: &impl Nvimapi, scrolls: Vec<uievent::Scroll>) {
    todo!()
}

fn do_set_scroll_regions(nvim: &impl Nvimapi, set_scroll_regions: Vec<uievent::SetScrollRegion>) {
    todo!()
}

fn do_puts(nvim: &impl Nvimapi, puts: Vec<uievent::Put>) {
    todo!()
}

fn do_highlight_sets(nvim: &impl Nvimapi, highlight_sets: Vec<uievent::HighlightSet>) {
    todo!()
}

fn do_cursor_gotos(nvim: &impl Nvimapi, cursor_gotos: Vec<uievent::CursorGoto>) {
    todo!()
}

fn do_eol_clears(nvim: &impl Nvimapi, eol_clears: Vec<uievent::EolClear>) {
    todo!()
}

fn do_clears(nvim: &impl Nvimapi, clears: Vec<uievent::Clear>) {
    todo!()
}

fn do_resizes(nvim: &impl Nvimapi, resizes: Vec<uievent::Resize>) {
    todo!()
}

fn do_update_sps(nvim: &impl Nvimapi, update_sps: Vec<uievent::UpdateSp>) {
    todo!()
}

fn do_update_bgs(nvim: &impl Nvimapi, update_bgs: Vec<uievent::UpdateBg>) {
    todo!()
}

fn do_update_fgs(nvim: &impl Nvimapi, update_fgs: Vec<uievent::UpdateFg>) {
    todo!()
}

fn do_chdirs(nvim: &impl Nvimapi, chdirs: Vec<uievent::Chdir>) {
    todo!()
}

fn do_screenshots(nvim: &impl Nvimapi, screenshots: Vec<uievent::Screenshot>) {
    todo!()
}

fn do_set_icons(nvim: &impl Nvimapi, set_icons: Vec<uievent::SetIcon>) {
    todo!()
}

fn do_set_titles(nvim: &impl Nvimapi, set_titles: Vec<uievent::SetTitle>) {
    todo!()
}

fn do_suspends(nvim: &impl Nvimapi, suspends: Vec<uievent::Suspend>) {
    todo!()
}

fn do_flushs(nvim: &impl Nvimapi, flushs: Vec<uievent::Flush>) {
    todo!()
}

fn do_visual_bells(nvim: &impl Nvimapi, visual_bells: Vec<uievent::VisualBell>) {
    todo!()
}

fn do_bells(nvim: &impl Nvimapi, bells: Vec<uievent::Bell>) {
    todo!()
}

fn do_mode_changes(nvim: &impl Nvimapi, mode_changes: Vec<uievent::ModeChange>) {
    todo!()
}

fn do_mouse_offs(nvim: &impl Nvimapi, mouse_offs: Vec<uievent::MouseOff>) {
    todo!()
}

fn do_mouse_ons(nvim: &impl Nvimapi, mouse_ons: Vec<uievent::MouseOn>) {
    todo!()
}

fn do_busy_stops(nvim: &impl Nvimapi, busy_stops: Vec<uievent::BusyStop>) {
    todo!()
}

fn do_busy_starts(nvim: &impl Nvimapi, busy_starts: Vec<uievent::BusyStart>) {
    todo!()
}

fn do_update_menus(nvim: &impl Nvimapi, update_menus: Vec<uievent::UpdateMenu>) {
    todo!()
}

fn do_mode_info_sets(nvim: &impl Nvimapi, mode_info_sets: Vec<uievent::ModeInfoSet>) {
    todo!()
}

fn do_option_sets(nvim: &impl Nvimapi, option_sets: Vec<uievent::OptionSet>) {
    todo!()
}
