
//! Advent of Code input file (down)loader
// by Rennex on 29-31.1.2023

// Do your "cargo run" inside day01/ or any other day.

//! # Usage
//!
//! Put this in your Cargo.toml:
//! ```
//! [dependencies]
//! input_downloader = { path = "../input_downloader" }
//! ```
//!
//! and in your main.rs:
//! ```
//! use input_downloader;
//! // just open or download the input:
//! let input = input_downloader::year(2022).get();
//! // or specify example inputs for testing:
//! let input = input_downloader::year(2022).example(
//! "FOO
//! BAR"
//! ).example("other test data").get();
//! ```
//!
//! The function returns the first example instead of the real input,
//! if you supply the `-e` argument on the command line.
//!
//! (Example: `cargo run -- -e`)

use std::fs;
use std::io::ErrorKind;
use std::process::Command;
use std::env;

fn extract_day_number(input: &str) -> Option<u32> {
    let day_index = input.find("day")?;
    let day_string = &input[day_index..];
    let day_number = day_string.chars().skip(3)     // skip past "day"
        .take_while(|c| c.is_digit(10))
        .fold(0, |day, c| day * 10 + c.to_digit(10).unwrap());
    Some(day_number)
}

fn day_from_cwd() -> u32 {
    let path = env::current_dir().expect("failed to get currentdir");
    let path = path.to_str().unwrap();
    extract_day_number(path)
        .expect("can't find day number in currentdir! It needs to include e.g. \"day01\".")
}


/// Opens and reads the file named `input` in the current dir.
/// If it's not there, it's downloaded and saved, and then read.
///
/// For downloading, the wanted day is automatically parsed from
/// the current dir (looking for e.g. `day01` in the path), and the
/// user's session cookie is read from `../.cookie`.
///
/// So your paths and files are expected to look like:
/// - adventofcode/2022/day01/input
/// - adventofcode/2022/day01/src/main.rs
/// - adventofcode/2022/.cookie
///
/// # Panics
/// Panics on unrecoverable errors.

pub fn get_input(year: u32) -> String {
    let data = fs::read_to_string("input");
    match data {
        Ok(s) => return s,
        Err(e) => {
            if e.kind() != ErrorKind::NotFound {
                panic!("error: {:?}", e);
            }
        }
    }

    let day = day_from_cwd();
    println!("Downloading input for day {day}...");

    let session_key = fs::read_to_string("../.cookie")
        .expect("can't read session key from \"../.cookie\"!");

    let status =
        Command::new("curl")
        .arg("-sO")
        .arg("--fail")      // exit with non-zero status unless http 200 ok
        .arg("--cookie")
        .arg(format!("session={session_key}"))
        .arg(format!("https://adventofcode.com/{year}/day/{day}/input"))
        .status()
        .expect("failed to run curl");

    if status.success() {
        fs::read_to_string("input").expect("failed to open downloaded file")
    } else {
        panic!("curl failed!");
    }
}


pub struct InputDownloader {
    year: u32,
    examples: Vec<String>
}

/// Returns an [InputDownloader] for the given year.
/// You may optionally add one or more example inputs using example().
/// Finally, call get() on it to get the input String.
pub fn year(year: u32) -> InputDownloader {
    InputDownloader {
        year,
        examples: Vec::new()
    }
}

fn example_num_from_env() -> Option<usize> {
    if let Some(e_arg) = env::args()
        .find(|arg| arg.starts_with("-e"))
    {
        let num_slice = &e_arg[2..];
        if num_slice.len() == 0 {
            return Some(0);
        }
        return num_slice.parse().ok();
    }
    None
}

impl InputDownloader {
    /// Adds an example input. May be called multiple times.
    pub fn example(mut self, text: &str) -> Self {
        self.examples.push(text.to_string());
        self
    }

    /// Returns the wanted input: normally the real input, or
    /// the first example if "-e" is specified on the command line.
    /// "-e2" returns the second example, etc.
    pub fn get(&self) -> String {
        if self.examples.len() > 0 {
            if let Some(mut num) = example_num_from_env() {
                if num != 0 {
                    num -= 1;
                }
                return self.examples[num].clone();
            }
        }
        get_input(self.year)
    }

    /// Returns the wanted input as a collection: normally the real
    /// input as the only String in a Vec, or all examples if "-e"
    /// is specified on the command line.
    /// "-e1" returns the first example alone, etc.
    pub fn get_all(&self) -> Vec<String> {
        if self.examples.len() > 0 {
            if let Some(num) = example_num_from_env() {
                if num != 0 {
                    let n = num - 1;
                    return Vec::from(&self.examples[n..=n]);
                }
                return self.examples.clone();
            }
        }
        Vec::from([get_input(self.year)])
    }

}
