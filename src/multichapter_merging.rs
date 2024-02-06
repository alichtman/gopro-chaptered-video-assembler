use std::{path::PathBuf, process};

use colored::Colorize;
use log::{error, info};
use normpath::PathExt;
// use predicates::path;

use crate::gopro::GoProChapteredVideoFile;

// Create "concat demux" input files
pub fn combine_multichapter_videos(
    multichapter_videos_sorted: std::collections::HashMap<u16, Vec<GoProChapteredVideoFile>>,
    output_dir: PathBuf,
) {
    if multichapter_videos_sorted.len() == 0 {
        info!("{}", "No multichapter videos to combine".blue().bold());
        return;
    }
    // Iterate through multichapter video map, and mp4-merge it.
    for video in multichapter_videos_sorted {
        let number = video.0;
        let mut paths_to_chapters = Vec::<PathBuf>::new();
        // TODO: Accumulate the chapters into a vec, then pass to mp4-merge
        for chapter in video.1 {
            paths_to_chapters.push(chapter.abs_path.clone());
            info!(
                "Concatenating chapter {:?} of video {}...",
                chapter.abs_path.to_str(),
                number
            );
        }
        let output_filename = generate_merged_chaptered_video_output_file_name(&output_dir, number);
        mp4_merge::join_files(&paths_to_chapters, &output_filename, |progress| {
            println!("Merging... {:.2}%", progress * 100.0);
        })
        .unwrap();
    }
}

fn generate_merged_chaptered_video_output_file_name(
    output_dir: &PathBuf,
    video_number: u16,
) -> PathBuf {
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
    output_file_name
}
