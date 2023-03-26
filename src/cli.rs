use colored::Colorize;
use log::info;
use std::{
    path::{Path, PathBuf},
    process,
};

use clap::Parser;

#[derive(Parser)]
#[command(author, version, about)]
pub struct Cli {
    /// Directory to parse video files from
    #[arg(short, long, value_name = "FILE")]
    pub gopro_video_dir: Option<PathBuf>,

    /// Directory to output video files to
    #[arg(short, long, value_name = "FILE")]
    pub output_dir: Option<PathBuf>,
}

/// Ensures a path is passed.
pub fn validate_args() -> Cli {
    let args = Cli::parse();
    // println!("Processing path: {:?}", args.gopro_video_dir);
    if args.gopro_video_dir.is_none() {
        eprintln!(
            "{} {}",
            "No GoPro video directory (--gopro_video_dir) provided.".red().bold(),
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

    info!("GoPro video directory: {:?}", args.gopro_video_dir);
    info!("Output directory: {:?}", args.output_dir);
    
    args
}
