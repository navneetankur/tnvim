#![feature(unix_mkfifo)]
use std::{os::unix::fs::{OpenOptionsExt as _, PermissionsExt as _}, path::PathBuf};
fn main() {
    #[cfg(debug_assertions)]
    {
        init_logger_debug();
        log_panics::init();
    }
    #[cfg(not(debug_assertions))]
    init_logger();
    tnvim::main();
}

const LOG_FIFO: &str = "/tmp/tnvim.log.fifo";
pub fn init_logger() {
    let folder = 
        if let Ok(folder) = std::env::var("XDG_CACHE_HOME") { PathBuf::from(folder) }
        else if let Some(mut home) = std::env::home_dir() {
            home.push(".cache");
            home
        } else {
            return;
        };
    let logfile_path = folder.join("tnvim.log");
    let Ok(logfile) = std::fs::OpenOptions::new().append(true).open(&logfile_path) else {
        println!("failed to open logfile: {}", logfile_path.display());
        return;
    };
    env_logger::builder()
        .filter_level(log::LevelFilter::Warn)
        .format_line_number(true)
        .format_file(true)
        .format_module_path(false)
        .format_level(false)
        .format_timestamp(None)
        .format_target(false)
        .target(env_logger::Target::Pipe(Box::new(logfile)))
        .init();
}
pub fn init_logger_debug() {
    if let Err(e) = std::os::unix::fs::mkfifo(LOG_FIFO, std::fs::Permissions::from_mode(0o600))
        && e.kind() != std::io::ErrorKind::AlreadyExists { panic!("{e}") };
    let fifo = match open_fifo(LOG_FIFO, true) {
        Ok(fifo) => fifo,
        Err(e) => {
            let errorno = std::io::Error::last_os_error().raw_os_error().unwrap();
            if errorno != nix::libc::ENXIO { panic!("failed to open fifo: {}\n{e:?}", LOG_FIFO); }
            temp_start_kitty(LOG_FIFO);
            open_fifo(LOG_FIFO, false).unwrap()
        },
    };
    env_logger::builder()
        .filter_level(log::LevelFilter::Debug)
        .format_line_number(true)
        .format_file(true)
        .format_module_path(false)
        // .format_source_path(false)
        .format_level(false)
        .format_timestamp(None)
        .format_target(false)
        .target(env_logger::Target::Pipe(Box::new(fifo)))
        .init();
}


fn open_fifo(path: impl AsRef<std::path::Path>, no_block: bool) -> std::io::Result<std::fs::File> {
    let mut oo = std::fs::OpenOptions::new();
    oo.write(true);
    if no_block { oo.custom_flags(nix::libc::O_NONBLOCK); }
    return oo.open(path);
}


#[allow(clippy::zombie_processes)]
fn temp_start_kitty(path: &str) {
    std::process::Command::new("kitty")
        .args(["--single-instance",
            "sh", "-c",
        ]).arg(format!("exec cat <> {}", path)).spawn().unwrap();
}

