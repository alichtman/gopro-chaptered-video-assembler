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
use std::io::Error;
use std::path::PathBuf;

use log::warn;

/// This struct represents a chaptered GoPro video file (what the camera writes to disk)
#[derive(Debug, Clone)]
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

pub fn parse_gopro_file(path: PathBuf) -> Result<GoProChapteredVideoFile, Error> {
    // println!("\n\nParsing file: {:?}", path);
    let filename = path.as_path().file_name().unwrap().to_str().unwrap();
    if path.is_dir() {
        return Err(Error::new(
            std::io::ErrorKind::InvalidData,
            format!("{} is a directory", filename),
        ));
    }
    let extension = path
        .as_path()
        .extension()
        .unwrap()
        .to_str()
        .unwrap()
        .to_lowercase();
    let prefix = filename.get(0..2).unwrap();

    if extension == "jpg" && (prefix == "GO" || prefix == "G0") {
        return Err(Error::new(
            std::io::ErrorKind::InvalidData,
            format!("{} is (likely) a GoPro image", filename),
        ));
    }
    if extension != "mp4" || (prefix != "GH" && prefix != "GX") {
        return Err(Error::new(
            std::io::ErrorKind::InvalidData,
            format!("Invalid file extension or prefix: {}", filename),
        ));
    }
    let video_number: u16 = match filename.get(4..8).unwrap().parse() {
        Ok(v) => v,
        Err(e) => {
            return Err(Error::new(
                std::io::ErrorKind::InvalidData,
                format!(
                    "Error parsing video number: {filename}
                n{e}"
                ),
            ));
        }
    };
    let chapter: u16 = match filename.get(2..4).unwrap().parse() {
        Ok(v) => v,
        Err(e) => {
            return Err(Error::new(
                std::io::ErrorKind::InvalidData,
                format!("Error parsing chapter number: {filename}\n{e}"),
            ));
        }
    };

    Ok(GoProChapteredVideoFile {
        abs_path: path.canonicalize().unwrap(),
        video_number,
        chapter,
    })
}

pub fn parse_gopro_files_directory(input_files: Vec<PathBuf>) -> Vec<GoProChapteredVideoFile> {
    let mut videos: Vec<GoProChapteredVideoFile> = Vec::new();
    for file in input_files {
        let gopro_file_metadata: GoProChapteredVideoFile = match parse_gopro_file(file) {
            Ok(gopro_file_metadata) => {
                // info!("Parsed GoPro Video File: {}", gopro_file_metadata);
                gopro_file_metadata
            }
            Err(e) => {
                warn!("Failed to parse GoPro video file: {}", e);
                continue;
            }
        };
        videos.push(gopro_file_metadata);
    }
    videos
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

// Assumes output_dir is a normalized directory path. Adds GoPro_{}.EXTENSION to the end of the path.
pub fn gen_output_path(output_dir: &PathBuf, video_number: u16, extension: &str) -> PathBuf {
    let mut output_path = PathBuf::from(output_dir);
    output_path.push(format!("GoPro_{}", video_number));
    output_path.set_extension(extension);
    output_path
}
