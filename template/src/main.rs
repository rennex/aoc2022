
// 0:00 start

#![allow(unused)]
use std::fmt::Display;

use input_downloader;
use indoc::indoc;

fn main() {
    let inputs = input_downloader::year(2022)
        .example(indoc!("
            FOO BAR
        "))
        .get_all();

    for input in inputs {
        solve(&input);
    }
}

fn solve(input: &String) {
    println!("input size = {}", input.len());
}
