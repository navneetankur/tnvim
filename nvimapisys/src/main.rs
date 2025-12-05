fn main() {
    let mut args = std::env::args();
    args.next();
    env_logger::Builder::new()
        .format_source_path(true)
        .format_level(false)
        .filter_level(log::LevelFilter::Debug)
        .format_target(false)
        // .format_timestamp(Some(env_logger::TimestampPrecision::Seconds))
        .format_timestamp(None)
        .init();
    if let Some(arg) = args.next() {
        if arg == "-g" {
            nvimapisys::build::main();
        }
    } else {
        nvimapisys::main();
    }
}
