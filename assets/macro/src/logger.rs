// use flexi_logger::{FileSpec, Logger, WriteMode};

pub fn init_logger(debug: bool) {
    let log_level = if debug {
        log::Level::max()
    } else {
        log::Level::Warn
    };

    if let Err(error) = simple_logger::init_with_level(log_level) {
        log::warn!("Error initializing logger: {}", error);
    }
}

// fn try_init_logger(
//     debug: bool,
// ) -> Result<flexi_logger::LoggerHandle, flexi_logger::FlexiLoggerError> {
//     Logger::try_with_str("assets_macro")
//         .unwrap()
//         .log_to_file(FileSpec::default())
//         .write_mode(WriteMode::BufferAndFlush)
//         .start()
// }
