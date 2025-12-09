use std::rc::Rc;
use crate::terminal;
use terminal::event::{KeyCode, KeyModifiers};
use log::{debug, info};
use nvimapi::Nvimapi;
use tokio::sync::mpsc::{self, error::TrySendError};
use crate::{TERM_INPUT_BUFFER_SIZE, app::App};

pub async fn input_from_term(this: Rc<App>, nvim: impl Nvimapi) {
    let (tx, mut rx) = mpsc::channel::<terminal::event::Event>(TERM_INPUT_BUFFER_SIZE);
    std::thread::spawn(|| crosscode_event_loop(tx));
    while let Some(event) = rx.recv().await {
        handle_event(&this, &nvim, event).await;
    }
    ;
}

async fn on_mouse(this: &App, nvim: &impl Nvimapi, mouse_event: terminal::event::MouseEvent) {
    todo!()
}

async fn on_paste(this: &App, nvim: &impl Nvimapi, paste: String) {
    todo!()
}

async fn on_resize(this: &App, nvim: &impl Nvimapi, w: u16, h: u16) {
    // todo send to nvim resize info.
}

async fn on_key(this: &App, nvim: &impl Nvimapi, key_event: terminal::event::KeyEvent) {
    use terminal::event::{KeyCode, KeyModifiers};
    // debug!("on key: {key_event:?}");
    if key_event.code == KeyCode::Char('c') && key_event.modifiers.contains(KeyModifiers::CONTROL) {
        super::exit();
    } else {
        if let Some(to_send) = to_nvim_input_key(key_event) {
            nvim.input(&to_send).await.unwrap();
        }
    }
}

async fn on_focus_lost(this: &App, nvim: &impl Nvimapi) {
    todo!()
}

async fn on_focus_gained(this: &App, nvim: &impl Nvimapi) {
    todo!()
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
        rv.push(c);
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
    return Some(rv);
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
