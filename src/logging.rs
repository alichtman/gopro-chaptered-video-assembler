extern crate xdg;
extern crate log;
use log::*;
use simplelog::*;
use std::fs::File;

const PROGRAM_NAME: &str = env!("CARGO_PKG_NAME");

pub fn initialize_logging() {
    let xdg_dirs = xdg::BaseDirectories::with_prefix(PROGRAM_NAME).unwrap();
    let log_file_path = xdg_dirs.place_cache_file(format!("{}.log", PROGRAM_NAME)).unwrap();
    CombinedLogger::init(vec![
        TermLogger::new(
            LevelFilter::Info,
            Config::default(),
            TerminalMode::Mixed,
            ColorChoice::Auto,
        ),
        WriteLogger::new(
            LevelFilter::Info,
            Config::default(),
            File::create(log_file_path).unwrap(),
        ),
    ])
    .unwrap();
}
