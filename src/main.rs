#![feature(path_file_prefix)]

mod cli;
mod ffmpeg;
mod filesystem;
mod gopro;
mod logging;
mod printing;
use crate::ffmpeg::concatenate_mp4s_from_demuxer_file;
use crate::filesystem::{append_path_to_demux_input_file, get_files_in_directory, print_file};
use crate::gopro::{gen_output_path, GoProChapteredVideoFile};
use crate::logging::initialize_logging;
use crate::printing::{get_confirmation_before_proceeeding, print_expected_output, print_header};
use std::fs::rename;

use clap::Parser;
use cli::CliArgs;
use colored::Colorize;
use filesystem::create_dir;
use gopro::parse_gopro_files_directory;
use log::{error, info};
use printing::print_remove_commands;
use std::path::PathBuf;
use std::process;

fn main() {
    initialize_logging();
    print_header();

    let args = CliArgs::parse();
    let input_dir = args
        .input_dir
        .clone()
        .unwrap()
        .canonicalize()
        .expect("Could not canonicalize input dir path");
    let input_files = filesystem::get_files_in_directory(input_dir.to_str().unwrap());
    if input_files.is_empty() {
        error!(
            "{} {}",
            "No files found in directory: ".red().bold(),
            input_dir.display()
        );
        process::exit(1);
    } else {
        info!(
            "Found {} files in directory: {}",
            input_files.len(),
            input_dir.as_os_str().to_string_lossy().blue().bold()
        );
    }

    // Extract data for each video file
    let videos = parse_gopro_files_directory(input_files);
    // Sort the videos by video number, preparing them to be "concat demuxed" by ffmpeg https://stackoverflow.com/a/11175851
    let mut multichapter_videos_sorted = gopro::sort_gopro_files(videos);
    // Filter out videos that only have one chapter to be renamed separately
    let mut single_chapter_videos = multichapter_videos_sorted.clone();
    single_chapter_videos.retain::<_>(|_k, v| v.len() == 1);
    // And then drop them from the multichapter videos map
    multichapter_videos_sorted.retain::<_>(|_k, v| v.len() > 1);

    let output_dir = create_dir(args.output_dir.clone().unwrap());

    // Show expected output for multichapter combinations and single chapter renames
    print_expected_output(
        single_chapter_videos.clone(),
        multichapter_videos_sorted.clone(),
    );
    match get_confirmation_before_proceeeding(args.auto_confirm_yes) {
        true => (),
        false => {
            info!("Exiting...");
            process::exit(0);
        }
    }

    combine_multichapter_videos(
        multichapter_videos_sorted.clone(),
        output_dir.clone(),
        args.clone(),
    );
    rename_single_chapter_videos(single_chapter_videos, output_dir, args.clone());
    print_remove_commands(multichapter_videos_sorted);
}

fn rename_single_chapter_videos(
    single_chapter_videos: std::collections::HashMap<u16, Vec<GoProChapteredVideoFile>>,
    output_dir: PathBuf,
    args: CliArgs,
) {
    for video in single_chapter_videos {
        let video_number = video.0;
        let video_path = video.1[0].abs_path.clone();
        let output_path = gen_output_path(&output_dir.clone(), video_number);
        info!(
            "Renaming {} to {}",
            video_path.to_string_lossy().green().bold(),
            output_path.to_string_lossy().blue().bold()
        );
        if args.dry_run {
            info!("Dry run, skipping rename!");
            continue;
        } else {
            rename(video_path, output_path).expect("Failed to rename file");
        }
    }
}

fn combine_multichapter_videos(
    multichapter_videos_sorted: std::collections::HashMap<u16, Vec<GoProChapteredVideoFile>>,
    output_dir: PathBuf,
    args: CliArgs,
) {
    // Create "concat demux" input files
    info!("Creating \"concat demux\" input files...");
    let concat_demuxer_input_files_dir = filesystem::create_temp_dir();
    for video in multichapter_videos_sorted {
        let video_number = video.0;
        for chapter in video.1 {
            // If output/GoPro_video_number.txt doesn't exist, create it.
            // Then append the abs_path of the chapter to the file.
            let output_path = gen_output_path(&concat_demuxer_input_files_dir, video_number);
            append_path_to_demux_input_file(output_path, chapter.abs_path)
                .expect("Failed to write to file");
        }
    }

    let input_files = get_files_in_directory(concat_demuxer_input_files_dir.to_str().unwrap());
    // Run ffmpeg concat demuxer on each input file
    for file in input_files {
        let concat_demuxer_input_file = PathBuf::from(&file);
        let video_number = file.file_prefix().unwrap().to_str().unwrap();
        let mut output_file_name = PathBuf::from(output_dir.clone());
        output_file_name.push(video_number);
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
