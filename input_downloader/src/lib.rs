
// Advent of Code input file loader
// by Rennex on 29.1.2023

// get_input() opens and reads the file named "input" in the
// current dir. If it's not there, it's downloaded and then read.
// Panics on unrecoverable errors.

// For downloading, the wanted day is automatically parsed from
// the currendir (looking for e.g. "day01" in the path), and the
// user's session cookie is read from "../.cookie".

// So your paths and files are expected to look like:
// adventofcode/2022/day01/input
// adventofcode/2022/day01/src/main.rs
// adventofcode/2022/.cookie

// Do your "cargo run" inside day01/

// # Usage

// Put this in your Cargo.toml:
// [dependencies]
// input_downloader = { path = "../input_downloader" }

// and in your main.rs:
//      use input_downloader::*;
//      let input = get_input(2022);


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
