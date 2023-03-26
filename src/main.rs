#![feature(path_file_prefix)]

mod cli;
mod ffmpeg;
mod filesystem;
mod gopro;
mod logging;
mod printing;
use crate::logging::initialize_logging;
use crate::cli::{validate_args};
use crate::ffmpeg::concatenate_mp4s_from_demuxer_file;
use crate::filesystem::{append_path_to_demux_input_file, get_files_in_directory, read_lines};
use crate::gopro::{parse_gopro_file, GoProChapteredVideoFile};
use crate::printing::print_box_header;
use std::fs::create_dir_all;

use colored::Colorize;
use log::{info, error};
use std::path::PathBuf;
use std::process;

// TODO: Finish integrating logging
// TODO: Configure logging file to XDG Cache directory

fn header() {
    let name = format!("{} {}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
    print_box_header(name);
}

fn main() {
    initialize_logging();
    header();

    let args = validate_args();
    let input_dir = args.input_dir.unwrap();
    let input_files = filesystem::get_files_in_directory(input_dir.to_str().unwrap());
    if input_files.is_empty() {
        error!(
            "{} {}",
            "No files found in directory: ".red().bold(),
            input_dir.to_str().unwrap()
        );
        process::exit(1);
    } else {
        info!("Found {} files in directory: {}", input_files.len(), input_dir.to_str().unwrap());
    }

    // Extract data for each video file
    let mut videos: Vec<GoProChapteredVideoFile> = Vec::new();
    for file in input_files {
        let gopro_file_metadata: GoProChapteredVideoFile = parse_gopro_file(file);
        info!("Parsed GoPro Video File: {}", gopro_file_metadata);
        videos.push(gopro_file_metadata);
    }
    // Sort the videos by video number, preparing them to be "concat demuxed" by ffmpeg https://stackoverflow.com/a/11175851
    let sorted_videos = gopro::sort_gopro_files(videos);

    // Create "concat demux" input files
    info!("Creating \"concat demux\" input files...");
    let concat_demuxer_input_files_dir = filesystem::create_temp_dir();
    for video in sorted_videos {
        let video_number = video.0;
        for chapter in video.1 {
            info!("Video: {:#?}", chapter);

            // If output/GoPro_video_number.txt doesn't exist, create it.
            // Then append the abs_path of the chapter to the file.
            let output_path = PathBuf::from(format!(
                "{}/GoPro_{}.txt",
                concat_demuxer_input_files_dir.to_string_lossy(),
                video_number
            ));
            // println!("Concat Demuxer Input files path: {:?}", output_path);
            append_path_to_demux_input_file(output_path, chapter.abs_path)
                .expect("Failed to write to file");
        }
    }

    let output_dir = PathBuf::from(args.output_dir.unwrap().as_os_str().to_str().unwrap().trim());
    create_dir_all(output_dir.clone()).expect("Failed to create output dir");

    let input_files = get_files_in_directory(concat_demuxer_input_files_dir.to_str().unwrap());
    // Run ffmpeg concat demuxer on each input file
    info!("Creating {} output files...", input_files.len());
    for file in input_files {
        let concat_demuxer_input_file = PathBuf::from(&file);
        let video_number = file.file_prefix().unwrap().to_str().unwrap();
        let mut output_file_name = PathBuf::from(output_dir.clone());
        output_file_name.push(video_number);
        output_file_name.set_extension("mp4");
        info!("Concat Demuxer Input file: {:?}", concat_demuxer_input_file);
        println!("Creating output file {} from:", output_file_name.to_string_lossy().blue().bold());
        if let Ok(lines) = read_lines(concat_demuxer_input_file.clone())
        {
            for line in lines {
                if let Ok(l) = line {
                    println!("{}", l.green().bold());
                }
            }
        }
        concatenate_mp4s_from_demuxer_file(concat_demuxer_input_file, output_file_name)
    }
}
