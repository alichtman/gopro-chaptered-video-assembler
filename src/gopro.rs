//
// https://community.gopro.com/s/article/GoPro-Camera-File-Naming-Convention?language=en_US
//
// There are three types of videos:
// 1. Single
// 2. Chaptered
// 3. Looping

// I only care about single and chaptered videos.
// The general format is: GXYYZZZZ.mp4, where:
//                        X is the encoding type (X for HEVC, H for AVC .... yes, I know)
//                        YY is the chapter number
//                        ZZZZ is the video number
//
// Single Video: GH011234.mp4 (first video)
// No more processing is needed here, since it's the first (and only) video.

// Chaptered Video: GH011234.mp4 (first video)
//                  GH021234.mp4 (second video)
//                  ...
// Chaptered videos require concatenation of... all chapters

use std::collections::HashMap;
use std::path::PathBuf;

/// This struct represents the fully concatenated video file we are going to create. This file is
/// composed of all of the GoProChapteredVideoFiles that have the same video_number.
#[derive(Debug)]
pub struct GoProVideo {
    pub abs_path: PathBuf,
    // encoding: Encoding,
    pub video_number: u16,
}

/// This struct represents a chaptered GoPro video file (what the camera writes to disk)
#[derive(Debug)]
pub struct GoProChapteredVideoFile {
    pub abs_path: PathBuf,
    pub video_number: u16,
    pub chapter: u16,
}

impl std::fmt::Display for GoProChapteredVideoFile {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "\nabs_path: {}\nvideo_number: {}\nchapter: {}\n",
            self.abs_path.display(),
            self.video_number,
            self.chapter
        )
    }
}

pub fn parse_gopro_file(path: PathBuf) -> GoProChapteredVideoFile {
    // println!("\n\nParsing file: {:?}", path);
    let filename = path.as_path().file_name().unwrap().to_str().unwrap();
    let video_number = filename.get(4..8).unwrap();
    let video_number: u16 = match video_number.parse() {
        Ok(v) => v,
        Err(e) => panic!("Error parsing video number: {}", e),
    };
    let chapter = filename.get(2..4).unwrap();
    let chapter: u16 = match chapter.parse() {
        Ok(v) => v,
        Err(e) => panic!("Error parsing chapter: {}", e),
    };

    GoProChapteredVideoFile {
        abs_path: path.canonicalize().unwrap(),
        video_number,
        chapter,
    }
}

pub fn sort_gopro_files(
    videos: Vec<GoProChapteredVideoFile>,
) -> HashMap<u16, Vec<GoProChapteredVideoFile>> {
    let mut video_number_to_subvideos_mapping: HashMap<u16, Vec<GoProChapteredVideoFile>> =
        HashMap::new();

    for video in videos {
        video_number_to_subvideos_mapping
            .entry(video.video_number)
            .or_insert(Vec::new())
            .push(video);
    }

    video_number_to_subvideos_mapping
}
