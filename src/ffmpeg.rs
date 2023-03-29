use core::panic;
use std::process::Command;
use std::{path::PathBuf, process};

use colored::Colorize;
use filetime::set_file_times;
use log::{error, info};
use normpath::PathExt;

use crate::filesystem::{get_last_modified_time, print_file};
use crate::gopro::GoProVideo;
use crate::{
    cli::CliArgs,
    filesystem::{self, append_path_to_demux_input_file},
    gopro::{gen_output_path, GoProChapteredVideoFile},
};

pub fn concatenate_mp4s_from_demuxer_file(
    input_file: PathBuf,
    output_file: PathBuf,
    cli: CliArgs,
    time_to_set: filetime::FileTime,
) {
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
        .arg(output_file.clone());
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

    set_file_times(output_file.as_path(), time_to_set, time_to_set).unwrap();
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
    // Run ffmpeg concat demuxer on each input file
    let ffmpeg_demuxer_inputs = create_ffmpeg_demuxer_input_files(multichapter_videos_sorted);
    process_ffmpeg_demuxer_input_files(ffmpeg_demuxer_inputs, output_dir, args);
}

fn process_ffmpeg_demuxer_input_files(
    gopro_videos_to_be_demuxed: Vec<GoProVideo>,
    output_dir: PathBuf,
    args: CliArgs,
) {
    for gopro_video in gopro_videos_to_be_demuxed {
        let video_number = gopro_video.video_number;

        // This chunk where I generate output_file_name needs to be cleaned up
        let mut output_file_name = match PathBuf::from(output_dir.clone()).normalize() {
            Ok(path) => path,
            Err(e) => {
                error!("Could not normalize output directory path: {}", e);
                process::exit(1);
            }
        };
        output_file_name.push(format!("GoPro_{}", video_number.to_string()));
        let mut output_file_name = output_file_name.as_path().to_path_buf();
        output_file_name.set_extension("mp4");

        info!(
            "Concat Demuxer Input file: {:?}",
            gopro_video.demuxer_input_file
        );
        println!(
            "Creating output file {} from:",
            output_file_name.to_string_lossy().blue().bold()
        );
        print_file(&gopro_video.demuxer_input_file);

        concatenate_mp4s_from_demuxer_file(
            gopro_video.demuxer_input_file,
            output_file_name,
            args.clone(),
            gopro_video.mtime,
        );
    }
}

// Returns a vector of GoProVideos. GoProVideo.demuxer_input_file is the path to the demuxer input file
fn create_ffmpeg_demuxer_input_files(
    multichapter_videos_sorted: std::collections::HashMap<u16, Vec<GoProChapteredVideoFile>>,
) -> Vec<GoProVideo> {
    let mut gopro_multichapter_videos_to_demux = Vec::<GoProVideo>::new();
    let ffmpeg_demuxer_files_dir = filesystem::create_temp_dir();
    info!(
        "Creating \"concat demux\" input files in {}...",
        ffmpeg_demuxer_files_dir.display()
    );
    for video in multichapter_videos_sorted {
        let video_number = video.0;
        let ffmpeg_demuxer_filepath =
            gen_output_path(&ffmpeg_demuxer_files_dir, video_number, "demux.txt");
        let last_modified_time_of_first_chapter = get_last_modified_time(&video.1[0].abs_path);
        for chapter in video.1 {
            let chapter_abs_path = chapter.abs_path.clone();
            info!(
                "Writing {:?} to concat demuxer file: {:?}",
                chapter_abs_path,
                ffmpeg_demuxer_filepath.clone()
            );

            // Escape single quotes in path with weird ffmpeg concat demuxer syntax
            let chapter_abs_path = PathBuf::from(
                chapter_abs_path
                    .to_path_buf()
                    .to_string_lossy()
                    .replace("'", "'\\''"),
            );
            append_path_to_demux_input_file(ffmpeg_demuxer_filepath.clone(), chapter_abs_path)
                .expect("Failed to write to concat demuxer file: {ffmpeg_demuxer_filepath}");
        }
        gopro_multichapter_videos_to_demux.push(GoProVideo {
            video_number,
            demuxer_input_file: ffmpeg_demuxer_filepath,
            mtime: last_modified_time_of_first_chapter,
        })
    }
    gopro_multichapter_videos_to_demux
}
