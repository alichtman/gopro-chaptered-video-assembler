extern crate colored;
// extern crate uuid;
use colored::*;
use log::info;
use normpath::PathExt;
use std::fs::create_dir_all;
use std::path::PathBuf;
use std::process;

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