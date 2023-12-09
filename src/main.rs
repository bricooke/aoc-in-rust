mod day1;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;
mod day19;
mod day2;
mod day20;
mod day21;
mod day22;
mod day23;
mod day24;
mod day25;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;

use day1::*;
use day10::*;
use day11::*;
use day12::*;
use day13::*;
use day14::*;
use day15::*;
use day16::*;
use day17::*;
use day18::*;
use day19::*;
use day2::*;
use day20::*;
use day21::*;
use day22::*;
use day23::*;
use day24::*;
use day25::*;
use day3::*;
use day4::*;
use day5::*;
use day6::*;
use day7::*;
use day8::*;
use day9::*;
use std::{env, fs::read_to_string};

static FUNCTIONS: &'static [(fn(&str) -> String, fn(&str) -> String)] = &[
    (day1_part1, day1_part2),
    (day2_part1, day2_part2),
    (day3_part1, day3_part2),
    (day4_part1, day4_part2),
    (day5_part1, day5_part2),
    (day6_part1, day6_part2),
    (day7_part1, day7_part2),
    (day8_part1, day8_part2),
    (day9_part1, day9_part2),
    (day10_part1, day10_part2),
    (day11_part1, day11_part2),
    (day12_part1, day12_part2),
    (day13_part1, day13_part2),
    (day14_part1, day14_part2),
    (day15_part1, day15_part2),
    (day16_part1, day16_part2),
    (day17_part1, day17_part2),
    (day18_part1, day18_part2),
    (day19_part1, day19_part2),
    (day20_part1, day20_part2),
    (day21_part1, day21_part2),
    (day22_part1, day22_part2),
    (day23_part1, day23_part2),
    (day24_part1, day24_part2),
    (day25_part1, day25_part2),
];

#[cfg(feature = "dhat-heap")]
#[global_allocator]
static ALLOC: dhat::Alloc = dhat::Alloc;

fn input(day: u8) -> String {
    read_to_string(format!("./input/day{}.txt", day))
        .expect(format!("Should have been able to read input/day{}.txt", day).as_str())
}

fn run_day(day: u8) {
    let input = input(day);
    let (part1, part2) = FUNCTIONS[(day - 1) as usize];
    println!("Day {day}, part 1: {}", part1(input.as_str()));
    println!("Day {day}, part 2: {}", part2(input.as_str()));
}

fn main() {
    #[cfg(feature = "dhat-heap")]
    let _profiler = dhat::Profiler::new_heap();

    let args: Vec<String> = env::args().collect();
    let day_as_string = args.get(1);
    let day_to_run;
    if day_as_string.is_some() {
        day_to_run = Some(
            day_as_string
                .unwrap()
                .parse::<u8>()
                .expect("Expected the arg to be the day as an integer"),
        );
    } else {
        day_to_run = None;
    }

    let days: Vec<u8> = (1..=25).collect();

    if day_to_run.is_some() && days.contains(&day_to_run.unwrap()) {
        run_day(day_to_run.unwrap());
    } else {
        for day in days {
            run_day(day);
        }
    }
}
