#[allow(unused_imports)]
use log::{debug, trace};
use nvimapi::{Nvimapi, Pairs, TryFromValue, uievent};
use rmpv::Value;
use serde::Deserialize;
use suffixes::CastIt;
use crate::{app::App, nvim::data::{RgbAttrs}, };
use nvimapi::Color as NColor;
use crate::terminal::CursorShape;



pub(super) async fn do_set_title(app: &App, _: &impl Nvimapi, events: Vec<uievent::SetTitle>) {
    let st = &events[0];
    app.terminal.set_title(&st.title).unwrap();
}
pub(super) async fn do_default_colors_set(app: &App, _: &impl Nvimapi, events: Vec<uievent::DefaultColorsSet>) {
    let colors = &events[0];
    let fg = NColor::from(colors.rgb_fg);
    let bg = NColor::from(colors.rgb_bg);
    let sp = NColor::from(colors.rgb_sp);
    trace!("color set: {bg:?},{fg:?}");
    let mut data = app.nvimdata.borrow_mut();
    data.color_set.fg = fg;
    data.color_set.bg = bg;
    data.color_set.sp = sp;
    data.apply_hl_id_forced(0, &app.terminal);
    drop(data);
}
pub(super) async fn do_hl_attr_define(this: &App, _: &impl Nvimapi, events: Vec<uievent::HlAttrDefine>) {
    let mut data = this.nvimdata.borrow_mut();
    let rgb_attrs = &mut data.hl_attrs;

    let last_id = events.last().unwrap().id.u();
    
    if rgb_attrs.len() < last_id {
        rgb_attrs.resize(last_id, RgbAttrs::default());
        // unsafe resize would not work as it contains a string.
    }

    for hl_define in events {
        let rgb_attr = RgbAttrs::deserialize(Value::Map(hl_define.rgb_attrs.inner)).unwrap();
        // debug!("hlid: {}", hl_define.id);
        rgb_attrs[hl_define.id.u()] = rgb_attr;
    }
    drop(data);
}
pub(super) async fn do_grid_resize(this: &App, _: &impl Nvimapi, events: Vec<uievent::GridResize>) {
    let mut data = this.nvimdata.borrow_mut();
    let size = &events[0];
    assert_eq!(size.grid, 1);
    data.nvim_size.w = size.width.u16();
    data.nvim_size.h = size.height.u16();
    data.surface = grid::Grid::new(size.height.u(), size.width.u());
    drop(data);
}
pub(super) async fn do_grid_clear(app: &App, _nvim: &impl Nvimapi, events: Vec<uievent::GridClear>) {
    assert_eq!(events[0].grid, 1, "I only clear whole screen assuming there is only one grid.");
    let mut data = app.nvimdata.borrow_mut();
    data.surface.iter_mut().for_each(|v| *v = Default::default());
    data.apply_hl_id(0, &app.terminal);
    app.terminal.clear_screen().unwrap();
    trace!("grid_clear");
}
pub(super) async fn do_grid_cursor_goto(this: &App, _nvim: &impl Nvimapi, events: Vec<uievent::GridCursorGoto>) {
    trace!("cursor goto");
    for grid_cursor_goto in events {
        let col = grid_cursor_goto.col.u16();
        let row = grid_cursor_goto.row.u16();
        this.terminal.move_cursor(col, row).unwrap();
        this.set_cursor(col, row);
    }
}
pub(super) async fn do_grid_line(app: &App, _nvim: &impl Nvimapi, events: Vec<uievent::GridLine>) {
    trace!("grid_line");
    let mut current_hl_id = 1;
    let mut data = app.nvimdata.borrow_mut();
    for line in events {
        let mut col = line.col_start.u16();
        let row = line.row.u16();
        app.terminal.move_cursor(col, row).unwrap();
        for cell  in line.data.into_iter() {
            let Value::Array(cell) = cell else {unreachable!()};
            let mut items = cell.into_iter();
            let text = items.next().unwrap();
            let Value::String(text) = text else {unreachable!()};
            let text = text.into_str().unwrap_or_else(|| String::from("□"));
            // debug!("text: {text}");
            if let Some(hl_id) = items.next() {
                let hl_id = hl_id.as_u64().unwrap().u16();
                current_hl_id = hl_id;
                data.apply_hl_id(current_hl_id, &app.terminal);
            } 
            let repeat = items.next().map(|v| v.as_i64().unwrap()).unwrap_or(1);
            // repeat should be signed as it's possible for neovim to send repeat = 0.
            for _ in 0..(repeat-1) {
                app.terminal.print(&text).unwrap();
                let gcell = crate::nvim::data::Cell {
                    char_: text.clone(),
                    hl: current_hl_id.u16(),
                };
                data.surface[(row.u(), col.u())] = gcell;
                col += 1;
            }
            if repeat > 0 {
                app.terminal.print(&text).unwrap();
                let gcell = crate::nvim::data::Cell {
                    char_: text,
                    hl: current_hl_id.u16(),
                };
                data.surface[(row.u(), col.u())] = gcell;
                col += 1;
            }
        }
        // debug!("g: {grid},r: {row}, c: {col}, t: {buffer}");
    }
    app.terminal.move_cursor(data.cursor.pos.col, data.cursor.pos.row).unwrap();
    drop(data);
}
pub(super) async fn do_flush(app: &App, _nvim: &impl Nvimapi, _events: Vec<uievent::Flush>) {
    trace!("flush");
    app.terminal.flush().unwrap();
}
pub(super) async fn do_grid_scroll(app: &App, _nvim: &impl Nvimapi, events: Vec<uievent::GridScroll>) {
    log::trace!("grid_scroll");
    let mut data = app.nvimdata.borrow_mut();
    for scroll_event in events {
        let scroll = scroll_event.rows;
        if scroll > 0 {
            for row in (scroll_event.top + scroll)..scroll_event.bot {
                handle_scroll_row(app, &mut data, &scroll_event, row);
            }
        } else {
            // order of iter has to be reversed or row will overwrite the value from prev loop.
            for row in (scroll_event.top..(scroll_event.bot+scroll)).rev() {
                handle_scroll_row(app, &mut data, &scroll_event, row);
            }
        }
        // debug!("g:{}, t:{}, b:{}, l:{}, r:{}, r:{}, c:{}", scroll_event.grid, scroll_event.top, scroll_event.bot, scroll_event.left, scroll_event.right, scroll_event.rows, scroll_event.cols);
    }
    drop(data);
}

fn handle_scroll_row(app: &App, data: &mut std::cell::RefMut<'_, super::Data>, scroll_event: &uievent::GridScroll, row: i64) {
    let scroll = scroll_event.rows;
    let row = row.u();
    let Some(new_row) = row.checked_sub_signed(scroll.isize()) else {return;};
    // let Ok(new_row) = usize::try_from(new_row) else {continue;};
    app.terminal.move_cursor(scroll_event.left.u16(), new_row.u16()).unwrap();
    // debug!("move {} to {}.", row, new_row);
    for col in scroll_event.left..scroll_event.right {
        let col = col.u();
        let cell = core::mem::take(&mut data.surface[(row.u(), col)]);
        // let Some(cell) = cell else {continue;};
        data.apply_hl_id(cell.hl, &app.terminal);
        app.terminal.print(&cell.char_).unwrap();
        data.surface[(new_row, col)] = cell;
    }
}
pub(super) async fn do_mode_change(app: &App, _nvim: &impl Nvimapi, mode_changes: Vec<uievent::ModeChange>) {
    // debug!("{mode_changes:?}");
    for mode in mode_changes {
        if let Some(cursor_shape) = app.nvimdata.borrow().mode_cursors.get(mode.mode_idx.u()) {
            app.terminal.set_cursor_shape(*cursor_shape).unwrap();
        }
    }
    // mode (normal, insert) has changed.
}
pub(super) async fn do_mode_info_set(app: &App, _nvim: &impl Nvimapi, mode_info_sets: Vec<uievent::ModeInfoSet>) {
    let _json = serde_json::to_string(&mode_info_sets).unwrap();
    // debug!("{json}");
    let mut mode_cursors = Vec::<CursorShape>::new();
    for mode_info in mode_info_sets {
        // I always assume that mode_info.enabled is true. Why would i not want cursor shaped.
        for cursor_style in mode_info.cursor_styles {
            // let Value::Map(cursor_style) = cursor_style else {unreachable!()};
            let cursor_style = Pairs::<String, Value>::try_from_value(cursor_style).unwrap();
            if let Some(cursor_shape) = cursor_style.get_for_key("cursor_shape") {
                let cursor_shape = cursor_shape.as_str().unwrap();
                let blink_on = cursor_style.get_for_key("blinkon").map(|v| v.as_i64().unwrap()).unwrap_or(0);
                let blink_off = cursor_style.get_for_key("blinkoff").map(|v| v.as_i64().unwrap()).unwrap_or(0);
                let _blink_wait = cursor_style.get_for_key("blinkwait").map(|v| v.as_i64().unwrap()).unwrap_or(0);
                let no_blink = blink_on == 0 || blink_off == 0 ;
                let shape = 
                    match cursor_shape {
                        "block" => {
                            if no_blink {CursorShape::Block}
                            else {CursorShape::BlockBlink}
                        },
                        "horizontal" => {
                            if no_blink {CursorShape::UnderScore}
                            else {CursorShape::UnderScoreBlink}
                        },
                        "vertical" => {
                            if no_blink {CursorShape::Bar}
                            else {CursorShape::BarBlink}
                        },
                        shape => {
                            log::error!("cursor shape: {shape} unimplemented.");
                            CursorShape::Block
                        },
                    };
                mode_cursors.push(shape);
            } else {
                mode_cursors.push(CursorShape::Block);
            }
            // debug!("{cursor_shape:?}");

        }
    }
    // debug!("cshapes: {mode_cursors:?}");
    app.nvimdata.borrow_mut().mode_cursors = mode_cursors;
}
// new idea.
// even better.
// On focus lost, get the current tabpage and save it.
// On focus gained, switch nvim to the given tabpage.
// obviously other tnvim instances will be ignoring the change in tabpage as it's not focused.
//
// each tnvim instance is connected to a tabpage.
// It only listens to updates when nvim switches to it's own tabpage.
// Ignores otherwise. When a ui is focused it broadcasts it.
// when focus comes back to a tnvim. It broadcast this info and then switches tabpage to it's own
// and start listening to updates again.
