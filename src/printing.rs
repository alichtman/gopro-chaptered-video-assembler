use std::io::Write;

use colored::Colorize;
use log::info;

use crate::gopro::GoProChapteredVideoFile;

// This code sucks! Can't handle any multiline inputs, and looks seriously clunky.
pub fn print_box_header(text: String) {
    let mut header: String = "╔".to_string();
    for _ in 0..text.len() + 2 {
        header.push_str("═");
    }
    header.push_str("╗");
    header.push_str("\n║ ");
    header.push_str(&text);
    header.push_str(" ║\n╚");
    for _ in 0..text.len() + 2 {
        header.push_str("═");
    }
    header.push_str("╝");
    println!("{}", header.blue().bold());
}

pub fn print_header() {
    let name = format!("{} {}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
    print_box_header(name);
}

pub fn get_confirmation_before_proceeeding(skip_confirmation: bool) -> bool {
    if skip_confirmation {
        return true;
    }
    let mut input = String::new();
    print!("{} ", "Proceed? (y/n)".yellow().bold());
    std::io::stdout().flush().unwrap();
    std::io::stdin().read_line(&mut input).unwrap();
    if input.trim().to_string().to_lowercase().starts_with("y") {
        return true;
    } else {
        return false;
    }
}

pub fn print_expected_output(
    single_chapter_videos: std::collections::HashMap<u16, Vec<GoProChapteredVideoFile>>,
    multichapter_videos_sorted: std::collections::HashMap<u16, Vec<GoProChapteredVideoFile>>,
) {
    let mut total_chapters_to_combine = 0;
    let total_videos_to_output = multichapter_videos_sorted.len();
    for (_key, value) in multichapter_videos_sorted.clone() {
        total_chapters_to_combine += value.len();
    }
    info!(
        "Found {} video(s) with {} total chapters to combine",
        total_videos_to_output.to_string().blue().bold(),
        total_chapters_to_combine.to_string().blue().bold()
    );
    info!("{:#?}", multichapter_videos_sorted);
    info!(
        "And {} single chapter videos to move / rename",
        single_chapter_videos.len().to_string().blue().bold()
    );
}

pub fn print_remove_commands(
    multichapter_videos: std::collections::HashMap<u16, Vec<GoProChapteredVideoFile>>,
) {
    println!("Run the following command to remove the merged chapters");
    for (_key, chapters) in multichapter_videos {
        for chapter in chapters {
            println!("rm {}", chapter.abs_path.to_str().unwrap().blue().bold());
        }
    }
}
