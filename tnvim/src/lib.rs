use core::time::Duration;
use std::{os::unix::net::UnixStream, process::Command, rc::Rc};
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

fn attach(nvim: &impl Nvimapi,w: u16, h: u16) {
    use nvimapi::NvimapiNr;
    nvim.nr().ui_attach(w.into(), h.into(), nvimapi::Pairs::from_iter2([
        (nvimapi::UiOptions::ExtLinegrid, true),
    ])).unwrap();
}
const TERM_INPUT_BUFFER_SIZE :usize = 5;
pub fn server() -> String {
    let mut socket_file = std::env::var("XDG_RUNTIME_DIR").unwrap_or_else(|_|String::from("/tmp"));
    socket_file.push_str("/tnvim-server.s");
    socket_file
}
pub fn main(mut args: std::env::Args) {
    let app = App::default();
    setup(&app.terminal);
    let rt = LocalRuntime::new().unwrap();
    let rt = Rc::new(rt);
    let enter = rt.enter();
    args.next(); //program name
    rt.block_on(main_async(rt.clone(), app, args));
    drop(enter);
}
async fn main_async(rt: Rc<LocalRuntime>, _app: App, args: std::env::Args) {
    debug!("hello world");
    let app = Rc::new(App::default());
    let (starter, nvim,) = start_nvim_manager(app.clone(), rt.clone(), args);
    rt.spawn_local(term::input_from_term(app, nvim));
    starter.await;
    before_exit();
}

fn start_nvim_manager(app: Rc<App>, rt: Rc<LocalRuntime>, mut args: std::env::Args) -> (impl Future, impl Nvimapi) {
    use nvimapi::NvimapiNr;
    let socket_path = server();
    let stream = UnixStream::connect(&socket_path);
    let mut new_nvim = false;
    let stream =
        if let Ok(stream) = stream { stream }
        else {
            let _ = std::fs::remove_file(&socket_path);
            start_nvim(&socket_path);
            let mut connection = None;
            for _ in 0..100 {
                if let Ok(con) = UnixStream::connect(&socket_path) {
                    connection = Some(con);
                    break;
                } else {
                    std::thread::sleep(Duration::from_millis(10));
                }
            }
            new_nvim = true;
            connection.unwrap_or_else(|| panic!("failed to connect to: {socket_path}"))
        };
    let (task, nvim) = nvimapi::manager::start(app, rt, stream.try_clone().unwrap(), stream);
    let cwd = std::env::current_dir().unwrap_or_else(|_| std::path::PathBuf::from("."));
    if let Some(arg) = args.next() {
        if !new_nvim { nvim.nr().command("tabnew").unwrap(); }
        nvim.nr().command(&format!("edit {}/{arg}", cwd.display())).unwrap();
    }
    for arg in args {
        nvim.nr().command(&format!("edit {}/{arg}", cwd.display())).unwrap();
    }
    (task,nvim)
}

#[allow(clippy::zombie_processes)]
fn start_nvim(socket_path: &str) {
    let _command = Command::new("nvim")
        .args([
            "--listen", socket_path,
            "--headless",
        ]).spawn().unwrap();
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
    let _ = terminal::disable_bracketed_paste();
}

fn setup(term: &Terminal) {
    setup_exit();
    set_panic_hook();
    term.enter_alternate_screen().unwrap();
    term.enable_raw_mode().unwrap();
    term.enable_mouse_events().unwrap();
    term.enable_focus_events().unwrap();
    term.enable_bracketed_paste().unwrap();
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
