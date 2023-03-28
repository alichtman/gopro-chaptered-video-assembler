use std::{path::PathBuf};

use clap::Parser;

#[derive(Parser, Clone, Debug)]
#[clap(
    author = "Aaron Lichtman",
    version,
    about = "Assembles all chaptered GoPro video files in a directory into 'complete' files.\nWritten by: Aaron Lichtman\nSource: https://github.com/alichtman/gopro-chaptered-video-assembler"
)]
pub struct CliArgs {
    /// Directory to parse video files from
    #[arg(short, long, value_name = "DIRECTORY", required = true)]
    pub input_dir: Option<PathBuf>,

    /// Directory to output video files to
    #[arg(short, long, value_name = "DIRECTORY", required = true)]
    pub output_dir: Option<PathBuf>,

    /// Dry run. Does not write any files.
    #[arg(short, long, default_value = "false")]
    pub dry_run: bool,

    /// Auto-confirm yes to all prompts
    #[arg(short = 'y', long = "yes", default_value = "false")]
    pub auto_confirm_yes: bool,
}