#![feature(path_file_prefix)]

mod cli;
mod filesystem;
mod gopro;
mod logging;
mod multichapter_merging;
mod printing;
use crate::gopro::{gen_output_path, GoProChapteredVideoFile};
use crate::logging::initialize_logging;
use crate::multichapter_merging::combine_multichapter_videos;
use crate::printing::{get_confirmation_before_proceeeding, print_expected_output, print_header};
use std::fs::rename;

use clap::Parser;
use cli::CliArgs;
use colored::Colorize;
use filesystem::normalize_and_create_if_needed;
use gopro::parse_gopro_files_directory;
use log::{error, info};
use printing::print_remove_commands;
use std::path::PathBuf;
use std::process;

fn main() {
    initialize_logging();
    print_header();
    let args = CliArgs::parse();
    // print!("{:#?}", args);

    // Canonicalize input path up front. We don't handle the output path until later to avoid creating the output path if the user cancels the operation.
    let input_dir = args
        .input
        .clone()
        .unwrap()
        .canonicalize()
        .expect("Could not canonicalize input dir path. Does it exist?");

    actually_do_things_with_input_and_output_paths(input_dir, args);
}

fn actually_do_things_with_input_and_output_paths(input_dir: PathBuf, args: CliArgs) {
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

    // TODO: Ensure all videos are valid mp4s. (#10)
    // println!("{:?}", videos);

    // Sort the videos by video number, preparing them to be combined by mp4-merge
    let mut multichapter_videos_sorted = gopro::sort_gopro_files(videos);
    // Filter out videos that only have one chapter to be renamed separately
    let mut single_chapter_videos = multichapter_videos_sorted.clone();
    single_chapter_videos.retain::<_>(|_k, v| v.len() == 1);
    // And then drop them from the multichapter videos map
    multichapter_videos_sorted.retain::<_>(|_k, v| v.len() > 1);

    // Show expected output for multichapter combinations and single chapter renames
    print_expected_output(
        single_chapter_videos.clone(),
        multichapter_videos_sorted.clone(),
        args.copy_single_chapter_instead_of_renaming.clone(),
    );
    match get_confirmation_before_proceeeding(args.auto_confirm_yes) {
        true => (),
        false => {
            info!("Exiting...");
            process::exit(0);
        }
    }
    let output_dir = normalize_and_create_if_needed(args.output.clone().unwrap());

    combine_multichapter_videos(multichapter_videos_sorted.clone(), output_dir.clone());

    if args.copy_single_chapter_instead_of_renaming {
        print!("Copying single chapter videos instead of renaming");
        copy_single_chapter_videos(single_chapter_videos, output_dir, args.clone());
    } else {
        print!("Renaming single chapter videos");
        rename_single_chapter_videos(single_chapter_videos, output_dir, args.clone());
    }

    // Only print the remove commands if we combined any multichapter videos
    if multichapter_videos_sorted.len() > 0 {
        print_remove_commands(multichapter_videos_sorted);
    }
}

// There's some needless code duplication here. Could be cleaner

fn rename_single_chapter_videos(
    single_chapter_videos: std::collections::HashMap<u16, Vec<GoProChapteredVideoFile>>,
    output_dir: PathBuf,
    args: CliArgs,
) {
    for video in single_chapter_videos {
        let video_number = video.0;
        let video_path = video.1[0].abs_path.clone();
        let output_path = gen_output_path(&output_dir, video_number, "mp4");
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

fn copy_single_chapter_videos(
    single_chapter_videos: std::collections::HashMap<u16, Vec<GoProChapteredVideoFile>>,
    output_dir: PathBuf,
    args: CliArgs,
) {
    for video in single_chapter_videos {
        let video_number = video.0;
        let video_path = video.1[0].abs_path.clone();
        let output_path = gen_output_path(&output_dir, video_number, "mp4");
        info!(
            "Copying {} to {}",
            video_path.to_string_lossy().green().bold(),
            output_path.to_string_lossy().blue().bold()
        );
        if args.dry_run {
            info!("Dry run, skipping copy!");
            continue;
        } else {
            std::fs::copy(video_path, output_path).expect("Failed to copy file");
        }
    }
}
