use std::io::{Write, stdout};
use crossterm::{ExecutableCommand as _, QueueableCommand, terminal};
use log::{debug, trace};
use nvimapi::{Nvimapi, Pairs, uievent};
use rmpv::Value;
use serde::Deserialize;
use suffixes::CastIt;
use crate::{app::App, nvim::data::RgbAttrs, };
use nvimapi::Color as NColor;
use crossterm::cursor;



pub(super) async fn do_set_title(_: &App, _: &impl Nvimapi, events: Vec<uievent::SetTitle>) {
    let st = &events[0];
    stdout().execute(terminal::SetTitle(&st.title)).unwrap();
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
        rgb_attrs[event.id.u()] = rgb_attr;
    }
    drop(data);
}
pub(super) async fn do_grid_resize(this: &App, _: &impl Nvimapi, events: Vec<uievent::GridResize>) {
    let mut data = this.nvimdata.borrow_mut();
    for grid in events {
        let saved_grid = data.grids.entry(grid.grid.u16()).or_default();
        saved_grid.width = grid.width.u16();
        saved_grid.height = grid.height.u16();
    }
    drop(data);
}
pub(super) async fn do_grid_clear(this: &App, nvim: &impl Nvimapi, events: Vec<uievent::GridClear>) {
    debug!("grid_clear");
}
pub(super) async fn do_grid_cursor_goto(this: &App, nvim: &impl Nvimapi, events: Vec<uievent::GridCursorGoto>) {
    for grid_cursor_goto in events {
        let col = grid_cursor_goto.col.u16();
        let row = grid_cursor_goto.row.u16();
        stdout().queue(cursor::MoveTo(col, row)).unwrap();
        this.set_cursor(col, row);
    }
}
pub(super) async fn do_grid_line(this: &App, nvim: &impl Nvimapi, events: Vec<uievent::GridLine>) {
    use crossterm::style::Print;
    trace!("grid_line");
    let mut stdout = stdout();
    let mut hl_id = 1;
    for line in events {
        let _grid = line.grid;
        stdout.queue(cursor::MoveTo(line.col_start.u16(), line.row.u16())).unwrap();
        for cell in line.data {
            let Value::Array(cell) = cell else {unreachable!()};
            let mut items = cell.into_iter();
            let text = items.next().unwrap();
            let text = text.as_str().unwrap();
            let hl_id = items.next();
            let repeat = items.next().map(|v| v.as_i64().unwrap()).unwrap_or(1);
            for _ in 0..repeat {
                stdout.queue(Print(text)).unwrap();
            }
        }
    }
    let data = this.nvimdata.borrow();
    stdout.queue(cursor::MoveTo(data.cursor.col, data.cursor.row)).unwrap();
    drop(data);
}
pub(super) async fn do_flush(this: &App, nvim: &impl Nvimapi, events: Vec<uievent::Flush>) {
    stdout().flush().unwrap();
}
