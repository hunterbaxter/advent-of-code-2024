use std::{env, fs};

extern crate aoc_runner;

#[macro_use]
extern crate aoc_runner_derive;

pub mod day1;

aoc_lib! { year = 2024 }

pub fn read_file(folder: &str, day: usize) -> String {
    let cwd = env::current_dir().unwrap();
    let filepath = cwd.join(folder).join(format!("day{day}.txt"));
    let f = fs::read_to_string(filepath);
    f.expect("could not open input file")
}
