use std::path::PathBuf;
use std::process::Command;

use log::{info, error};

pub fn concatenate_mp4s_from_demuxer_file(input_file: PathBuf, output_file: PathBuf) {
    info!("Concatenating mp4s from {} to create {}...", input_file.display(), output_file.display());
    let program = "ffmpeg";
    let mut command = Command::new(&program);
    command
    .arg("-f")
    .arg("concat")
    .arg("-safe")
    .arg("0")
    .arg("-i")
    .arg(input_file)
    .arg("-c")
    .arg("copy")
    .arg(output_file);
    info!("Running command: {:?}", command);
    let output = command.spawn().unwrap().wait_with_output().unwrap();

    info!("status: {}", output.status);
    info!("stdout: {}", String::from_utf8_lossy(&output.stdout));
    info!("stderr: {}", String::from_utf8_lossy(&output.stderr));

    // if ffmpeg doesn't run successfully, scream and die 
    if !output.status.success() {
        error!("ffmpeg failed to concatenate mp4s from demuxer file");
        panic!("ffmpeg failed to concatenate mp4s from demuxer file");
    }
}
