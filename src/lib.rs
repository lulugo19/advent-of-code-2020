use std::{error::Error, fmt};

extern crate aoc_runner;

#[macro_use]
extern crate aoc_runner_derive;

#[macro_use]
extern crate lazy_static;
extern crate regex;

pub mod day1;
pub mod day2;
pub mod day3;
pub mod day4;
pub mod day5;
pub mod day6;
pub mod day7;
pub mod day8;
pub mod day9;

#[derive(Debug)]
pub struct NoSolutionErr {}
impl std::fmt::Display for NoSolutionErr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("no solution")
    }
}
impl Error for NoSolutionErr {}

aoc_lib! { year = 2020 }
