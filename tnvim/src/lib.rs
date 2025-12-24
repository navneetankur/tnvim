use core::time::Duration;
use std::{io::stdout, os::unix::net::UnixStream, rc::Rc};
use log::debug;
use nvimapi::Nvimapi;
mod terminal;
pub mod error;
use terminal::Terminal;
use tokio::runtime::LocalRuntime;
use crate::app::App;
mod app;
mod nvim;
mod term;
use rustc_hash::FxHashMap as HashMap;

fn attach(nvim: &impl Nvimapi,w: u16, h: u16) {
    use nvimapi::NvimapiNr;
    nvim.nr().ui_attach(w.into(), h.into(), nvimapi::Pairs::from_iter([
        (nvimapi::UiOptions::ExtLinegrid, true),
    ])).unwrap();
}

const TERM_INPUT_BUFFER_SIZE :usize = 5;

// pub const SERVER: &str = "/run/user/1000/nvim-server-temp.s";
pub const SERVER: &str = "/run/user/1000/nvim-server.s";
pub fn main() {
    let app = App::default();
    setup(&app.terminal);
    let rt = LocalRuntime::new().unwrap();
    let rt = Rc::new(rt);
    let _enter = rt.enter();
    rt.block_on(main_async(rt.clone(), app));
    drop(_enter);
}
async fn main_async(rt: Rc<LocalRuntime>, app: App) {
    debug!("hello world");
    let app = Rc::new(App::default());
    let (starter, nvim,) = start_nvim(app.clone(), rt.clone());
    rt.spawn_local(term::input_from_term(app, nvim));
    starter.await;
    before_exit();
}

fn start_nvim(app: Rc<App>, rt: Rc<LocalRuntime>) -> (impl Future, impl Nvimapi) {
    let stream = UnixStream::connect(SERVER).expect(SERVER);
    let (task, nvim) = nvimapi::manager::start(app, rt, stream.try_clone().unwrap(), stream);
    return (task,nvim);
}

pub extern "C" fn term_signal(_: core::ffi::c_int) {
    exit();
}
pub fn exit() {
    before_exit();
    std::process::exit(0);
}
fn before_exit() {
    terminal::leave_alternate_screen();
    terminal::disable_raw_mode();
    let _ = terminal::disable_mouse_events();
    let _ = terminal::disable_focus_events();
}

fn setup(term: &Terminal) {
    setup_exit();
    set_panic_hook();
    term.enter_alternate_screen().unwrap();
    term.enable_raw_mode().unwrap();
    term.enable_mouse_events().unwrap();
    term.enable_focus_events().unwrap();
}

fn set_panic_hook() {
    let hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |panic_info| {
        hook(panic_info);
        exit();
    }));
}

fn setup_exit() {
    use nix::sys::signal;

    let term_signals = [
        signal::SIGINT,   // Interrupt from keyboard (Ctrl+C)
        signal::SIGTERM,  // Termination signal
        signal::SIGHUP,   // Hangup detected on controlling terminal or death of controlling process
        signal::SIGQUIT,  // Quit from keyboard (Ctrl+\)
        signal::SIGABRT   // Abort signal from abort()
    ];
    for tsignal in term_signals {
        unsafe { signal::signal(tsignal, signal::SigHandler::Handler(term_signal)).unwrap() };
    }

}
