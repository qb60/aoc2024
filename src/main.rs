#![allow(dead_code)]

use std::{fmt::Display, fs};

mod multimap;
mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;

fn get_file(path: &str) -> String {
    fs::read_to_string(path).unwrap()
}

fn main() {
    // day("day1", "data/1.txt", day1::day);
    // day("day1_2", "data/1.txt", day1::day_2);
    // day("day2", "data/2.txt", day2::day);
    // day("day2_2", "data/2.txt", day2::day_2);
    // day("day3", "data/3.txt", day3::day);
    // day("day3_2", "data/3.txt", day3::day_2);
    // day("day4", "data/4.txt", day4::day);
    // day("day4_2", "data/4.txt", day4::day_2);
    // day("day5", "data/5.txt", day5::day);
    // day("day5_2", "data/5.txt", day5::day_2);
    // day("day6", "data/6.txt", day6::day);
    // day("day6_2", "data/6.txt", day6::day_2);
    // day("day7", "data/7.txt", day7::day);
    // day("day7_2", "data/7.txt", day7::day_2);
    // day("day8", "data/8.txt", day8::day);
    // day("day8_2", "data/8.txt", day8::day_2);
    day("day9", "data/9.txt", day9::day);
    day("day9_2", "data/9.txt", day9::day_2);
}

fn day<R: Display, F: Fn(&str) -> R>(label: &str, input_filename: &str, day_func: F) {
    let input = get_file(input_filename);
    let output = day_func(&input);
    println!("{} = {}", label, output);
}
