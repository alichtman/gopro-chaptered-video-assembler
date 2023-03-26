use std::path::PathBuf;
use std::process::{Command};

pub fn concatenate_mp4s_from_demuxer_file(input_file: PathBuf, output_file: PathBuf) {
    println!("Concatenating mp4s from demuxer file...");
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
    println!("Running command: {:?}", command);
    let output = command.spawn().unwrap().wait_with_output().unwrap();

    println!("status: {}", output.status);
    println!("stdout: {}", String::from_utf8_lossy(&output.stdout));
    println!("stderr: {}", String::from_utf8_lossy(&output.stderr));

    // if exit status isn't ok, panic 
    if !output.status.success() {
        panic!("ffmpeg failed to concatenate mp4s from demuxer file");
    }
}
