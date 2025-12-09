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
pub(super) async fn do_default_colors_set(this: &App, _: &impl Nvimapi, events: Vec<uievent::DefaultColorsSet>) {
    let colors = &events[0];
    let fg = NColor::from(colors.rgb_fg);
    let bg = NColor::from(colors.rgb_bg);
    let sp = NColor::from(colors.rgb_sp);
    let mut data = this.nvimdata.borrow_mut();
    data.color_set.fg = fg;
    data.color_set.bg = bg;
    data.color_set.sp = sp;
    drop(data);
}
pub(super) async fn do_hl_attr_define(this: &App, _: &impl Nvimapi, events: Vec<uievent::HlAttrDefine>) {
    let mut data = this.nvimdata.borrow_mut();
    let rgb_attrs = &mut data.hl_attrs;

    let last_id = events.last().unwrap().id.u();
    
    if rgb_attrs.len() <= last_id {
        rgb_attrs.resize(last_id + 1, RgbAttrs::default());
        // unsafe resize would not work as it contains a string.
    }

    for event in events {
        let rgb_attr = RgbAttrs::deserialize(Value::Map(event.rgb_attrs.inner)).unwrap();
        debug!("hlid: {}", event.id);
        rgb_attrs[event.id.u()] = rgb_attr;
    }
    drop(data);
}
pub(super) async fn do_grid_resize(this: &App, _: &impl Nvimapi, events: Vec<uievent::GridResize>) {
    let mut data = this.nvimdata.borrow_mut();
    for grid in events {
        let saved_grid = data.grids.entry(grid.grid.u16()).or_default();
        saved_grid.size.w = grid.width.u16();
        saved_grid.size.h = grid.height.u16();
        debug!("resize: g: {}, {}x{}", grid.grid, grid.width, grid.height);
    }
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
    debug!("grid_line");
    let mut stdout = stdout();
    let mut hl_id = 1;
    let mut buffer = String::with_capacity(30);
    for line in events {
        let grid = line.grid;
        let grid_pos = app.grid(grid.u16()).pos();
        let col = grid_pos.col + line.col_start.u16();
        let row = grid_pos.row + line.row.u16();
        app.terminal.move_cursor(col, row).unwrap();
        // write!(stdout, "g:{_grid}").unwrap();
        buffer.clear();
        for cell in line.data {
            let Value::Array(cell) = cell else {unreachable!()};
            let mut items = cell.into_iter();
            let text = items.next().unwrap();
            let text = text.as_str().unwrap();
            // debug!("{text}");
            let hl_id = items.next();
            let repeat = items.next().map(|v| v.as_i64().unwrap()).unwrap_or(1);
            for _ in 0..repeat {
                if text.is_empty() || text == " " {}
                else {
                    buffer.push_str(text);
                }
                app.terminal.print(text).unwrap();
            }
        }
        // debug!("g: {grid},r: {row}, c: {col}, t: {buffer}");
    }
    let data = app.nvimdata.borrow();
    app.terminal.move_cursor(data.cursor.pos.col, data.cursor.pos.row).unwrap();
    drop(data);
}
pub(super) async fn do_flush(this: &App, nvim: &impl Nvimapi, events: Vec<uievent::Flush>) {
    stdout().flush().unwrap();
}
pub(super) async fn do_msg_set_pos(app: &App, nvim: &impl Nvimapi, events: Vec<uievent::MsgSetPos>) {
    debug!("msg_set_pos");
    // pos of message window. on outer window wiz grid 1.
    for pos in events {
        // i am setting absolute position here, but nvim sent position relative to main grid. If
        // position of main grid changes will nvim send grid position again?. Should i save
        // relative position and calculate absolute position when needed? Needs to be seen.
        let main_pos = app.grid(MAIN_GRID.into()).pos();
        let row = pos.row;
        app.grid(pos.grid.u16()).set_pos(main_pos.col, row.u16());
        debug!("g: {}, r: {}, s: {}, s:{}", pos.grid, pos.row, pos.scrolled, pos.sep_char);
    }
}
pub(super) async fn do_grid_scroll(this: &App, nvim: &impl Nvimapi, events: Vec<uievent::GridScroll>) {
    log::info!("grid_scroll");
    for scroll in events {
        debug!("g:{}, t:{}, b:{}, l:{}, r:{}, r:{}, c:{}", scroll.grid, scroll.top, scroll.bot, scroll.left, scroll.right, scroll.rows, scroll.cols);
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
