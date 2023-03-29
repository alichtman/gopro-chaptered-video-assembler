use std::{path::PathBuf, process};
use std::process::Command;

use colored::Colorize;
use log::{error, info};
use normpath::PathExt;

use crate::filesystem::print_file;
use crate::{cli::CliArgs, gopro::{GoProChapteredVideoFile, gen_output_path}, filesystem::{self, append_path_to_demux_input_file, get_files_in_directory}};

pub fn concatenate_mp4s_from_demuxer_file(input_file: PathBuf, output_file: PathBuf, cli: CliArgs) {
    info!(
        "Concatenating mp4s from {} to create {}...",
        input_file.display(),
        output_file.display()
    );
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
    if cli.auto_confirm_yes {
        command.arg("-y");
    }
    info!("Running command: {:?}", command);
    if cli.dry_run {
        info!("Dry run, skipping ffmpeg command!");
        return;
    }
    let output = command.spawn().unwrap().wait_with_output().unwrap();

    // if ffmpeg doesn't run successfully, scream and die
    if !output.status.success() {
        info!("status: {}", output.status);
        info!("stdout: {}", String::from_utf8_lossy(&output.stdout));
        info!("stderr: {}", String::from_utf8_lossy(&output.stderr));
        error!("ffmpeg failed to concatenate mp4s from demuxer file");
        panic!("ffmpeg failed to concatenate mp4s from demuxer file");
    }
}

// Create "concat demux" input files
pub fn combine_multichapter_videos(
    multichapter_videos_sorted: std::collections::HashMap<u16, Vec<GoProChapteredVideoFile>>,
    output_dir: PathBuf,
    args: CliArgs,
) {
    if multichapter_videos_sorted.len() == 0 {
        info!("{}", "No multichapter videos to combine".blue().bold());
        return;
    }
    let ffmpeg_demuxer_files_dir = filesystem::create_temp_dir();
    info!("Creating \"concat demux\" input files in {}...", ffmpeg_demuxer_files_dir.display());
    for video in multichapter_videos_sorted {
        let video_number = video.0;
        for chapter in video.1 {
            let ffmpeg_demuxer_filepath =
                gen_output_path(&ffmpeg_demuxer_files_dir, video_number, "demux.txt");
            info!(
                "Writing to concat demuxer file: {:?}",
                ffmpeg_demuxer_filepath
            );
            append_path_to_demux_input_file(ffmpeg_demuxer_filepath, chapter.abs_path)
                .expect("Failed to write to concat demuxer file: {ffmpeg_demuxer_filepath}");
        }
    }

    let input_files = get_files_in_directory(ffmpeg_demuxer_files_dir.to_str().unwrap());
    // Run ffmpeg concat demuxer on each input file
    for concat_demuxer_input_file in input_files {
        let video_number = concat_demuxer_input_file
            .file_prefix()
            .unwrap()
            .to_str()
            .unwrap();
        let mut output_file_name = match PathBuf::from(output_dir.clone()).normalize() {
            Ok(path) => path,
            Err(e) => {
                error!("Could not normalize output directory path: {}", e);
                process::exit(1);
            }
        };
        output_file_name.push(video_number);
        let mut output_file_name = output_file_name.as_path().to_path_buf();
        output_file_name.set_extension("mp4");
        info!("Concat Demuxer Input file: {:?}", concat_demuxer_input_file);
        println!(
            "Creating output file {} from:",
            output_file_name.to_string_lossy().blue().bold()
        );
        print_file(&concat_demuxer_input_file);
        concatenate_mp4s_from_demuxer_file(
            concat_demuxer_input_file,
            output_file_name,
            args.clone(),
        );
    }
}
