use colored::Colorize;
use log::info;
use std::{path::PathBuf, process};

use clap::Parser;

#[derive(Parser)]
#[clap(author = "Aaron Lichtman", version, about="Assembles all chaptered GoPro video files in a directory into 'complete' files.\nWritten by: Aaron Lichtman\nSource: https://github.com/alichtman/gopro-chaptered-video-assembler")]
pub struct Cli {
    /// Directory to parse video files from
    #[arg(short, long, value_name = "DIRECTORY")]
    pub input_dir: Option<PathBuf>,

    /// Directory to output video files to
    #[arg(short, long, value_name = "DIRECTORY")]
    pub output_dir: Option<PathBuf>,
}

/// Ensures a path is passed.
pub fn validate_args() -> Cli {
    let args = Cli::parse();
    // println!("Processing path: {:?}", args.input_dir);
    if args.input_dir.is_none() {
        eprintln!(
            "{} {}",
            "No GoPro video directory (--input_dir) provided."
                .red()
                .bold(),
            "Please provide a path to a directory containing GoPro video files."
        );
        process::exit(1);
    }

    if args.output_dir.is_none() {
        eprintln!(
            "{} {}",
            "No output directory (--output_dir) provided.".red().bold(),
            "Please provide a path for video output."
        );
        process::exit(1);
    }

    info!("GoPro video directory: {:?}", args.input_dir);
    info!("Output directory: {:?}", args.output_dir);

    args
}
