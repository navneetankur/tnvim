fn main() {
    env_logger::Builder::new()
        .filter_level(log::LevelFilter::Debug)
        .format_source_path(true)
        .format_target(false)
        .format_timestamp(None)
        .format_level(false)
        .init();
    tnvim::main();
}
