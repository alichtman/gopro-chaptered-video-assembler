use assert_cmd::prelude::*;
use merkle_hash::{Algorithm, Encodable, MerkleTree};
use std::path::PathBuf;
use std::process::Command;

extern crate fs_extra;
use std::fs;

pub(crate) fn get_hash_of_directory(dir: &PathBuf) -> Vec<u8> {
    let tree = MerkleTree::builder(dir.to_str().unwrap())
        .algorithm(Algorithm::Blake3)
        .hash_names(false)
        .build()
        .unwrap();
    let master_hash = tree.root.item.hash;
    master_hash
}

pub(crate) fn get_path_to_source_videos() -> PathBuf {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("tests/data/real_videos");
    path
}

pub(crate) fn get_path_to_test_output() -> PathBuf {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("tests/data/actual_output");
    path
}

pub(crate) fn get_path_to_expected_output() -> PathBuf {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("tests/data/expected_output");
    path
}

pub(crate) fn setup() {
    self::teardown();
    let _ = fs::create_dir(get_path_to_test_output());
}

pub(crate) fn teardown() {
    for entry in vec![get_path_to_test_output()] {
        let _ = fs::remove_dir_all(entry);
    }
}

pub(crate) fn print_directory_contents(dir: &PathBuf) {
    let paths = fs::read_dir(dir).unwrap();
    for path in paths {
        println!("{:?}", path.unwrap().path());
    }
}

#[test]
fn test_run_on_dir() {
    self::setup();
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    cmd.arg("--input")
        .arg(get_path_to_source_videos())
        .arg("--output")
        .arg(get_path_to_test_output())
        .arg("--yes")
        .arg("--copy-single-chapter-instead-of-rename");
    let output = cmd.unwrap();
    print!("{:#?}", output);
    print!("\n#### Actual output directory contents: ");
    print_directory_contents(&get_path_to_test_output());
    // Check that actual_output matches expected_output
    let expected_output_hash = get_hash_of_directory(&get_path_to_expected_output());
    let actual_output_hash = get_hash_of_directory(&get_path_to_test_output());

    assert!(expected_output_hash == actual_output_hash);
    print!(
        "\n{:#?} was the merkle hash for both the expected and actual",
        expected_output_hash.to_hex_string()
    );

    self::teardown();
}
