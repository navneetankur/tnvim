fn main() {
    env_logger::Builder::new()
        .format_source_path(true)
        .format_level(false)
        .filter_level(log::LevelFilter::Debug)
        .format_target(false)
        // .format_timestamp(Some(env_logger::TimestampPrecision::Seconds))
        .format_timestamp(None)
        .init();
    // nvimapisys::main();
    nvimapisys::main();
}
