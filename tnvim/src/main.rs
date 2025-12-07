#![feature(unix_mkfifo)]

use std::os::unix::fs::{OpenOptionsExt as _, PermissionsExt as _};
fn main() {
    // color_eyre::install().unwrap();
    // env_logger::Builder::new()
    //     .filter_level(log::LevelFilter::Debug)
    //     .format_source_path(true)
    //     .format_target(false)
    //     .format_timestamp(None)
    //     .format_level(false)
    //     .init();
    init_logger();
    log_panics::init();
    tnvim::main();
}
const LOG_FIFO: &str = "/home/navn/workspace/rust/tnvim/log.fifo";
pub fn init_logger() {
    // let fp = std::fs::File::create(LOG_FILE).unwrap();
    if let Err(e) = std::os::unix::fs::mkfifo(LOG_FIFO, std::fs::Permissions::from_mode(0o600)) {
        if e.kind() != std::io::ErrorKind::AlreadyExists { panic!("{e}") };
    }
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


fn temp_start_kitty(path: &str) {
    std::process::Command::new("kitty")
        .args(["--single-instance",
            "sh", "-c",
        ]).arg(format!("exec cat <> {}", path)).spawn().unwrap();
}

