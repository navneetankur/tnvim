use std::io::{Write, stdout};
use log::{debug, trace};
use nvimapi::{Nvimapi, Pairs, uievent};
use rmpv::Value;
use serde::Deserialize;
use suffixes::CastIt;
use crate::{app::App, nvim::{MAIN_GRID, data::RgbAttrs}, };
use nvimapi::Color as NColor;



pub(super) async fn do_set_title(app: &App, _: &impl Nvimapi, events: Vec<uievent::SetTitle>) {
    let st = &events[0];
    app.terminal.set_title(&st.title).unwrap();
}
pub(super) async fn do_default_colors_set(app: &App, _: &impl Nvimapi, events: Vec<uievent::DefaultColorsSet>) {
    let colors = &events[0];
    let fg = NColor::from(colors.rgb_fg);
    let bg = NColor::from(colors.rgb_bg);
    let sp = NColor::from(colors.rgb_sp);
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
    data.size.w = size.width.u16();
    data.size.h = size.height.u16();
    data.surface = grid::Grid::new(size.height.u(), size.width.u());
    drop(data);
}
pub(super) async fn do_grid_clear(app: &App, nvim: &impl Nvimapi, events: Vec<uievent::GridClear>) {
    assert_eq!(events[0].grid, 1, "I only clear whole screen assuming there is only one grid.");
    app.terminal.clear_screen().unwrap();
    // for clear in events {
    //     let grid = app.grid(clear.grid.u16());
    //     let row = grid.pos().row;
    //     let height = grid.size().unwrap().h;
    //     for i in 0..height {
    //         app.terminal.move_cursor(0, row + i).unwrap();
    //         app.terminal.clear_row().unwrap();
    //     }
    // }
    debug!("grid_clear");
}
pub(super) async fn do_grid_cursor_goto(this: &App, nvim: &impl Nvimapi, events: Vec<uievent::GridCursorGoto>) {
    for grid_cursor_goto in events {
        let col = grid_cursor_goto.col.u16();
        let row = grid_cursor_goto.row.u16();
        this.terminal.move_cursor(col, row).unwrap();
        this.set_cursor(col, row);
    }
}
pub(super) async fn do_grid_line(app: &App, nvim: &impl Nvimapi, events: Vec<uievent::GridLine>) {
    trace!("grid_line");
    let mut current_hl_id = 1;
    let mut buffer = String::with_capacity(30);
    let mut data = app.nvimdata.borrow_mut();
    for line in events {
        let mut col = 0 + line.col_start.u16();
        let row = 0 + line.row.u16();
        // debug!("row: {row}");
        app.terminal.move_cursor(col, row).unwrap();
        // write!(stdout, "g:{_grid}").unwrap();
        buffer.clear();
        for cell  in line.data.into_iter() {
            let Value::Array(cell) = cell else {unreachable!()};
            let mut items = cell.into_iter();
            let text = items.next().unwrap();
            let text = text.as_str().unwrap();
            // debug!("{text}");
            // let hl_id = items.next()
            //     .map(|v|v.as_u64().unwrap().u16())
            //     .unwrap_or(prev_hl_id)
            //     ;
            if let Some(hl_id) = items.next() {
                // debug!("hlid: {hl_id}");
                // debug!("hlid: {hl_id}, t: {text}");
                let hl_id = hl_id.as_u64().unwrap().u16();
                current_hl_id = hl_id;
                data.apply_hl_id(current_hl_id, &app.terminal);
            } 
            // prev_hl_id = hl_id;
            let repeat = items.next().map(|v| v.as_i64().unwrap()).unwrap_or(1);
            for _ in 0..repeat {
                if text.is_empty() || text == " " {}
                else {
                    buffer.push_str(text);
                }
                app.terminal.print(text).unwrap();
                let mut chars = text.chars();
                let char_ = chars.next().unwrap();
                assert!(chars.next().is_none(), "multiple chars can come. storing of chars in cells logic is wrong here.");
                let gcell = crate::nvim::data::Cell {
                    char_,
                    hl: current_hl_id.u16(),
                };
                // if text != " " {
                //     debug!("{row}, {col}, {}, {text}, {char_}, {}", col.u() + i.u(), ' ');
                // }
                data.surface[(row.u(), col.u())] = gcell;
                col += 1;
            }
        }
        // debug!("g: {grid},r: {row}, c: {col}, t: {buffer}");
    }
    app.terminal.move_cursor(data.cursor.pos.col, data.cursor.pos.row).unwrap();
    drop(data);
}
pub(super) async fn do_flush(this: &App, nvim: &impl Nvimapi, events: Vec<uievent::Flush>) {
    stdout().flush().unwrap();
}
pub(super) async fn do_grid_scroll(app: &App, nvim: &impl Nvimapi, events: Vec<uievent::GridScroll>) {
    log::trace!("grid_scroll");
    debug!("grid_scroll");
    let mut data = app.nvimdata.borrow_mut();
    let mut buffer = [0u8;4];
    for scroll_event in events {
        let scroll = scroll_event.rows;
        if scroll > 0 {
            for row in (scroll_event.top + scroll)..scroll_event.bot {
                handle_scroll_row(app, &mut data, &mut buffer, &scroll_event, row);
            }
        } else {
            // order of iter has to be reversed or row will overwrite the value from prev loop.
            for row in (scroll_event.top..(scroll_event.bot+scroll)).rev() {
                handle_scroll_row(app, &mut data, &mut buffer, &scroll_event, row);
            }
        }
        // debug!("g:{}, t:{}, b:{}, l:{}, r:{}, r:{}, c:{}", scroll_event.grid, scroll_event.top, scroll_event.bot, scroll_event.left, scroll_event.right, scroll_event.rows, scroll_event.cols);
    }
    drop(data);
}

fn handle_scroll_row(app: &App, data: &mut std::cell::RefMut<'_, super::Data>, buffer: &mut [u8; 4], scroll_event: &uievent::GridScroll, row: i64) {
    let scroll = scroll_event.rows;
    let row = row.u();
    let Some(new_row) = row.checked_sub_signed(scroll.isize()) else {return;};
    // let Ok(new_row) = usize::try_from(new_row) else {continue;};
    app.terminal.move_cursor(scroll_event.left.u16(), new_row.u16()).unwrap();
    // debug!("move {} to {}.", row, new_row);
    for col in scroll_event.left..scroll_event.right {
        let col = col.u();
        let cell = data.surface[(row.u(), col)];
        data.surface[(new_row.into(), col)] = cell;
        app.terminal.print(cell.char_.encode_utf8(buffer)).unwrap();
    }
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
pub(super) async fn do_win_pos(this: &App, nvim: &impl Nvimapi, events: Vec<uievent::WinPos>) {
    //pos of main/outer window.
    debug!("do win pos");
    for event in events {
        debug!("g:{}, w:{:?}, r: {}, c: {}, w:{}, h:{}", event.grid, event.win, event.startrow, event.startcol, event.width, event.height);
    }
}
