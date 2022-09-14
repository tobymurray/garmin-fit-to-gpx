use core::{num, panic};
use fitparser;
use fitparser::profile::MesgNum;
use std::fs::File;
use std::fs::{self, DirEntry};
use std::io::Read;
use std::path::{Path, PathBuf};

fn main() {
    let mut count = 0;
    let mut num_failed = 0;

    let paths = read_test_data();
    println!("There are {} files", paths.len());

    for path in paths {
        let path_clone = path.clone();
        let path_clone2 = path.clone();
        let path_text = path_clone.display();
        let mut fp = File::open(path).unwrap();
        let parser = fitparser::from_reader(&mut fp);
        if parser.is_err() {
            println!("FAILED: {:?}", parser.err());
            num_failed += 1;
            continue;
        }
        let mut contains_activity = false;
        for data in parser.unwrap() {
            if data.kind() == MesgNum::Activity {
                println!("File {}", path_text);
                count += 1;
                contains_activity = true;
            }
        }

        if contains_activity {
            let filename_clone = path_clone2.clone();
            let filename = filename_clone.file_name().unwrap().to_str().unwrap();
            fs::copy(
                path_clone2,
                Path::new("/home/toby/git/rust/fit-decoder/fit_files/activity_files")
                    .join(filename),
            )
            .unwrap();
        }
    }

    println!("{} files had activities, {} failed", count, num_failed);
}

fn read_test_data() -> Vec<PathBuf> {
    fs::read_dir("/home/toby/git/rust/fit-decoder/fit_files/UploadedFiles_0-_Part1")
        .unwrap()
        .map(|r| r.unwrap())
        .map(|d| d.path())
        .collect()
}
