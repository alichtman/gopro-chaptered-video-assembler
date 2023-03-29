extern crate colored;
// extern crate uuid;
use colored::*;
use log::info;
use normpath::PathExt;
use std::fs::{create_dir_all, File, OpenOptions};
use std::io::{self, BufRead, Error, Write};
use std::path::{Path, PathBuf};
use std::process;
use uuid::Uuid;

pub fn get_files_in_directory(path: &str) -> Vec<PathBuf> {
    let mut files: Vec<PathBuf> = Vec::new();
    let directory = PathBuf::from(path).read_dir();
    if directory.is_err() {
        eprintln!("{} {}", "Directory not found:".red().bold(), path);
        process::exit(1);
    }
    for entry in directory.unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        files.push(path);
    }
    files
}

pub fn append_path_to_demux_input_file(
    target_file: PathBuf,
    path_to_append_to_file: PathBuf,
) -> Result<(), Error> {
    let parent_dir = target_file.parent().clone().unwrap();
    create_dir_all(parent_dir)?;
    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .create(true)
        .open(target_file)
        .expect("Something went wrong!");

    // TODO: One day this will break when a path with `'` in it is used.
    writeln!(file, "file '{}'", path_to_append_to_file.to_str().unwrap())?;
    Ok(())
}

// Returns temp dir path after making sure it exists
pub fn create_temp_dir() -> PathBuf {
    let temp_dir = std::env::temp_dir();
    let temp_dir = temp_dir.to_str().unwrap();
    let temp_dir = format!("{}/gopro_utility/{}", temp_dir, Uuid::new_v4());
    create_dir_all(temp_dir.clone()).expect("Failed to create temp dir");
    PathBuf::from(temp_dir)
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
pub fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

pub fn normalize_and_create_if_needed(path: PathBuf) -> PathBuf {
    let mut normalized_path = match path.clone().normalize() {
        Ok(path) => path,
        Err(_) => {
            info!(
                "{} directory does not exist, attempting to create it now...",
                path.to_string_lossy().blue().bold()
            );
            let path = path;
            let normalized_path = create_dir(path.clone().to_path_buf()).normalize().unwrap();
            normalized_path
        }
    };

    if normalized_path.exists() {
        info!(
            "{} directory exists, using it...",
            normalized_path.as_path().to_string_lossy().blue().bold()
        );
        normalized_path = normalized_path
            .clone()
            .normalize()
            .expect("Could not canonicalize output dir path");
    }
    normalized_path.into_path_buf()
}

pub fn create_dir(path: PathBuf) -> PathBuf {
    create_dir_all(path.clone()).expect("Failed to create dir");
    path
}

pub fn print_file(concat_demuxer_input_file: &PathBuf) {
    if let Ok(lines) = read_lines(concat_demuxer_input_file.clone()) {
        for line in lines {
            if let Ok(l) = line {
                println!("{}", l.green().bold());
            }
        }
    }
}
