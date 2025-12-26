use std::rc::Rc;
use crate::terminal;
use terminal::event::{KeyCode, KeyModifiers};
#[allow(unused_imports)]
use log::{debug, trace};
use nvimapi::{Nvimapi, NvimapiNr};
use tokio::sync::mpsc::{self};
use crate::{TERM_INPUT_BUFFER_SIZE, app::App};

pub async fn input_from_term(this: Rc<App>, nvim: impl Nvimapi) {
    let (tx, mut rx) = mpsc::channel::<terminal::event::Event>(TERM_INPUT_BUFFER_SIZE);
    std::thread::spawn(|| crosscode_event_loop(tx));
    while let Some(event) = rx.recv().await {
        handle_event(&this, &nvim, event).await;
    }
    ;
}

async fn on_mouse(_app: &App, nvim: &impl Nvimapi, mouse_event: terminal::event::MouseEvent) {
    let (btn, action) = 
        match mouse_event.kind {
            crossterm::event::MouseEventKind::Down(mouse_button) => {
                let btn = btn_str(mouse_button);
                let action = "press";
                (btn, action)
            },
            crossterm::event::MouseEventKind::Up(mouse_button) => {
                let btn = btn_str(mouse_button);
                let action = "release";
                (btn, action)
            },
            crossterm::event::MouseEventKind::Drag(mouse_button) => {
                let btn = btn_str(mouse_button);
                let action = "drag";
                (btn, action)
            },
            crossterm::event::MouseEventKind::Moved => {
                let btn = "move";
                let action = "";
                (btn, action)
            },
            crossterm::event::MouseEventKind::ScrollDown => {
                let btn = "wheel";
                let action = "down";
                (btn, action)
            },
            crossterm::event::MouseEventKind::ScrollUp => {
                let btn = "wheel";
                let action = "up";
                (btn, action)
            },
            crossterm::event::MouseEventKind::ScrollLeft => {
                let btn = "wheel";
                let action = "left";
                (btn, action)
            },
            crossterm::event::MouseEventKind::ScrollRight => {
                let btn = "wheel";
                let action = "right";
                (btn, action)
            },
        };
    let mut mods = Vec::new();
    modifier_map(mouse_event.modifiers, &mut mods);
    let mods: String = mods.iter().collect();
    nvim.nr().input_mouse(btn, action, &mods, 1, mouse_event.row.into(), mouse_event.column.into()).unwrap();
    // nvim.input_mouse(btn, action, &mods, 1, mouse_event.row.into(), mouse_event.column.into()).await.unwrap();
    fn btn_str(btn: crossterm::event::MouseButton) -> &'static str {
        match btn {
            crossterm::event::MouseButton::Left => "left",
            crossterm::event::MouseButton::Right => "right",
            crossterm::event::MouseButton::Middle => "middle",
        }
    }
}

async fn on_paste(_: &App, nvim: &impl Nvimapi, paste: String) {
    log::trace!("paste: {paste}");
    nvim.nr().paste(&paste, true, -1).unwrap();
}

async fn on_resize(app: &App, nvim: &impl Nvimapi, w: u16, h: u16) {
    let attached = app.nvimdata.borrow().attached;
    if attached {
        nvim.nr().ui_try_resize(w.into(), h.into()).unwrap();
    }
    else {
        let current_tab = nvim.get_current_tabpage().await.unwrap();
        if let Some(my_tab) = &app.nvimdata.borrow().my_tab {
            nvim.nr().set_current_tabpage(my_tab).unwrap();
        }
        crate::attach(nvim, w, h);
        nvim.nr().ui_detach().unwrap();
        nvim.nr().set_current_tabpage(&current_tab).unwrap();
    }
    app.nvimdata.borrow_mut().ui_size = crate::nvim::data::Size { w, h };
}

async fn on_key(_: &App, nvim: &impl Nvimapi, key_event: terminal::event::KeyEvent) {
    trace!("on key: {key_event:?}");
    if let Some(to_send) = to_nvim_input_key(key_event) {
        nvim.nr().input(&to_send).unwrap();
        trace!("sent: {to_send}");
    }
}

async fn on_focus_lost(app: &App, nvim: &impl Nvimapi) {
    // nvim.nr().ui_set_focus(false).unwrap();
    nvim.nr().ui_detach().unwrap();
    app.nvimdata.borrow_mut().attached = false;
    let current_tab = nvim.get_current_tabpage().await.unwrap();
    app.nvimdata.borrow_mut().my_tab = Some(current_tab);
}
async fn on_focus_gained(app: &App, nvim: &impl Nvimapi) {
    // nvim.nr().ui_set_focus(true).unwrap();
    let my_tab = app.nvimdata.borrow().my_tab.clone();
    if let Some(my_tab) = my_tab {
        // some delay, so that the other ui which lost focus, can save it's current tab before I
        // change it.
        tokio::time::sleep(core::time::Duration::from_millis(10)).await;
        nvim.nr().set_current_tabpage(&my_tab).unwrap();
    }
    let size = app.nvimdata.borrow().ui_size.clone();
    crate::attach(nvim, size.w, size.h);
    app.nvimdata.borrow_mut().attached = true;
    // debug!("focus_gained");
}

fn to_nvim_input_key(key_event: terminal::event::KeyEvent) -> Option<String> {
    use terminal::event::{KeyCode};
    let mut rv = String::new();
    let mut modifiers = Vec::new();
    modifier_map(key_event.modifiers, &mut modifiers);

    if !modifiers.is_empty() {
        rv.reserve(modifiers.len() * 3);
        rv.push('<');
        for &modifier in modifiers.iter() {
            rv.push(modifier);
            rv.push('-');
        }
    }
    if let KeyCode::Char(c) = key_event.code {
        if c == '<' {
            rv.push_str("<LT>");
        } else { rv.push(c); }
    } else {
        let spc = special_key_map(key_event.code);
        if spc.is_empty() { return None }
        if modifiers.is_empty() { rv.push('<'); }
        else { rv.push('-') }
        rv.push_str(spc);
        if modifiers.is_empty() { rv.push('>'); }
    }
    if !modifiers.is_empty() {
        rv.push('>');
    }
    Some(rv)
}
fn special_key_map(code: KeyCode,) -> &'static str {
    match code {
        KeyCode::Backspace => "BS",
        KeyCode::Enter => "CR",
        KeyCode::Left => "Left",
        KeyCode::Right => "Right",
        KeyCode::Up => "Up",
        KeyCode::Down => "Down",
        KeyCode::Home => "Home",
        KeyCode::End => "End",
        KeyCode::PageUp => "PageUp",
        KeyCode::PageDown => "PageDown",
        KeyCode::Tab => "Tab",
        KeyCode::Delete => "Del",
        KeyCode::Insert => "Insert",
        KeyCode::Null => "Nul",
        KeyCode::Esc => "Esc",
        KeyCode::CapsLock => "CapsLock",
        KeyCode::ScrollLock => "ScrollLock",
        KeyCode::NumLock => "NumLock",
        KeyCode::PrintScreen => "PrintScreen",
        KeyCode::Pause => "Pause",
        KeyCode::Menu => "Menu",
        KeyCode::KeypadBegin => "KeypadBegin",
        KeyCode::BackTab => "BackTab",
        _ => "",
    }
}
fn modifier_map(modifiers: KeyModifiers, buffer: &mut Vec<char>) {
    let rv = buffer;
    if modifiers.contains(KeyModifiers::CONTROL) {
        rv.push('C');
    }
    if modifiers.contains(KeyModifiers::ALT) {
        rv.push('A');
    }
    if modifiers.contains(KeyModifiers::SHIFT) {
        rv.push('S');
    }
    if modifiers.contains(KeyModifiers::SUPER) {
        rv.push('D');
    }
}

fn crosscode_event_loop(tx: mpsc::Sender<terminal::event::Event>) {
    use terminal::event::read;
    use mpsc::error::TrySendError;
    loop {
        if let Err(e) =  tx.try_send(read().unwrap()) {
            if let TrySendError::Full(_) = e {
                log::error!("channel to main thread full dropping input.");
                continue;
            } else {
                debug!("channel to mt gone. exit loop.");
                break;
            }
        }
    }
}

async fn handle_event(this: &App, nvim: &impl Nvimapi, event: terminal::event::Event) {
    use terminal::event::Event;
    match event {
        Event::FocusGained => on_focus_gained(this, nvim).await,
        Event::FocusLost => on_focus_lost(this, nvim).await,
        Event::Key(key_event) => on_key(this, nvim, key_event).await,
        Event::Mouse(mouse_event) => on_mouse(this, nvim, mouse_event).await,
        Event::Paste(paste) => on_paste(this, nvim, paste).await,
        Event::Resize(w, h) => on_resize(this, nvim, w, h).await,
    }
}
