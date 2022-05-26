// #![allow(unused)] // silence unused warnings while exploring (to comment out)

mod argc;
mod error;

use argc::argc_app;
use clap::ArgMatches;
use error::AppError;
use std::{error::Error, path::PathBuf};
use colored::Colorize;
use walkdir::WalkDir;

const TOP_NUMS: usize = 5;

fn get_dir() -> PathBuf {
    println!("Please enter a directory to get information about:");
    let mut dir = String::new();
    std::io::stdin().read_line(&mut dir).unwrap();
    let dir = PathBuf::from(dir.trim());

    if !dir.is_dir() {
        println!("\"{}\" is not a directory", dir.display());
        std::process::exit(1);
    }

    dir
}

pub fn fit_4(size: u64) -> String {
    // if you have more efficient or prettier, please tell me
    match size {
        0..=9_999 => format!("{:.0}o", size as f64),
        10_000..=999_499 => format!("{:.0}Ko", (size as f64) / 1_000.0),
        999_500..=9_950_000 => format!("{:.1}Mo", (size as f64) / 1_000_000.0),
        9_950_001..=999_499_999 => format!("{:.0}Mo", (size as f64) / 1_000_000.0),
        999_500_000..=9_950_000_000 => format!("{:.1}Go", (size as f64) / 1_000_000_000.0),
        9_950_000_001..=999_499_999_999 => format!("{:.0}Go", (size as f64) / 1_000_000_000.0),
        999_500_000_000..=9_950_000_000_000 => format!("{:.1}To", (size as f64) / 1_000_000_000_000.0),
        9_950_000_000_001..=999_499_999_999_999 => format!("{:.0}To", (size as f64) / 1_000_000_000_000.0),
        999_500_000_000_000..=9_950_000_000_000_000 => format!("{:.1}Po", (size as f64) / 1_000_000_000_000_000.0),
        9_950_000_000_000_001..=999_499_999_999_999_935 => format!("{:.0}Po", (size as f64) / 1_000_000_000_000_000.0),
        _ => "huge".to_string(), // good enough to me
    }
}

struct Entry {
    path: PathBuf,
    size: u64,
}

struct Options {
    nums: usize,
}

impl Options {
    fn from_argc(argc: ArgMatches) -> Result<Options, AppError> {
        let nums: usize = match argc.value_of("nums") {
            None => TOP_NUMS,
            Some(nums) => nums.parse::<usize>().or_else(|_| Err(AppError::InvalidNumberOfFiles(nums.to_string())))?,
        };

        Ok(Options { nums })
    }

    /* fn get_path(argc: ArgMatches) -> Result<&'static str, AppError> {
        #[cfg(target_os = "windows")]
        let path: &str = match argc.value_of("path") {
            None => {
                let dir = get_dir();
                let dir = dir.to_str().unwrap_or(".\\");

                dir
            },
            Some(path) => path,
        };
        #[cfg(not(target_os = "windows"))]
        let path: &str = match argc.value_of("path") {
            None => {
                let dir = get_dir();
                let dir = dir.to_str().unwrap_or("./");
            },
            Some(path) => path,
        };

        Ok(path)
    } */
}

fn exec(options: Options) -> Result<(), Box<dyn Error>> {
    let mut total_size: u64 = 0;
    let mut total_numbers: u32 = 0;
    let mut tops: Vec<Entry> = Vec::with_capacity(options.nums + 1);
    let mut min_of_tops = 0;

    #[cfg(target_os = "windows")]
    let dir: PathBuf = get_dir();
    let dir: &str = dir.to_str().unwrap_or(".\\");

    #[cfg(not(target_os = "windows"))]
    let dir: PathBuf = get_dir();
    #[cfg(not(target_os = "windows"))]
    let dir: &str = dir.to_str().unwrap_or("./");

    for entry in WalkDir::new(dir).into_iter().filter_map(|e| e.ok()) {
        let path = entry.path();
        if path.is_file() && !entry.path_is_symlink() {
            total_numbers += 1;
            let size = entry.metadata()?.len();
            total_size += size;

            if min_of_tops < size {
                tops.push(Entry {
                    path: entry.path().to_path_buf(),
                    size,
                });
                tops.sort_by(|a, b| b.size.cmp(&a.size));
                if tops.len() > options.nums {
                    tops.pop();
                }
                min_of_tops = tops.last().map(|e| e.size).unwrap_or(0);
            }
        }
        // println!("{}", entry.path().display());
    }

    println!("Number of files {}, total size: {}", total_numbers, fit_4(total_size));
    println!("Top {} biggest files", tops.len());
    for Entry { size, path } in tops.iter() {
        println!("{:<4} - {}", fit_4(*size), path.to_string_lossy());
    }

    Ok(())
}

fn main() {
    let argc = argc_app().get_matches();
    
    let options = match Options::from_argc(argc) {
        Ok(options) => options,
        Err(ex) => {
            println!("ERROR parsing input {:?}", ex);
            return;
        }
    };
    
    println!("Welcome on {}! This tool will allow {} to see which files are {}.", "Wib".red(), "YOU".bold(), "bigger".bold().red());

    match exec(options) {
        Ok(_) => (),
        Err(ex) => {
            println!("ERROR - {}", ex);
        }
    }
}