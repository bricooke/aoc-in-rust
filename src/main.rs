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
use std::fs::read_to_string;

fn main() {
    let input =
        read_to_string("./input/day1.txt").expect("Should have been able to read input/day1.txt");
    println!("Day 1, part 1: {}", day1_part1(&input));
    println!("Day 1, part 2: {}", day1_part2(&input));

    let input =
        read_to_string("./input/day2.txt").expect("Should have been able to read input/day2.txt");
    println!("Day 2, part 1: {}", day2_part1(&input));
    println!("Day 2, part 2: {}", day2_part2(&input));

    let input =
        read_to_string("./input/day3.txt").expect("Should have been able to read input/day3.txt");
    println!("Day 3, part 1: {}", day3_part1(&input));
    println!("Day 3, part 2: {}", day3_part2(&input));

    let input =
        read_to_string("./input/day4.txt").expect("Should have been able to read input/day4.txt");
    println!("Day 4, part 1: {}", day4_part1(&input));
    println!("Day 4, part 2: {}", day4_part2(&input));

    let input =
        read_to_string("./input/day5.txt").expect("Should have been able to read input/day5.txt");
    println!("Day 5, part 1: {}", day5_part1(&input));
    println!("Day 5, part 2: {}", day5_part2(&input));

    let input =
        read_to_string("./input/day6.txt").expect("Should have been able to read input/day6.txt");
    println!("Day 6, part 1: {}", day6_part1(&input));
    println!("Day 6, part 2: {}", day6_part2(&input));

    let input =
        read_to_string("./input/day7.txt").expect("Should have been able to read input/day7.txt");
    println!("Day 7, part 1: {}", day7_part1(&input));
    println!("Day 7, part 2: {}", day7_part2(&input));

    let input =
        read_to_string("./input/day8.txt").expect("Should have been able to read input/day8.txt");
    println!("Day 8, part 1: {}", day8_part1(&input));
    println!("Day 8, part 2: {}", day8_part2(&input));

    let input =
        read_to_string("./input/day9.txt").expect("Should have been able to read input/day9.txt");
    println!("Day 9, part 1: {}", day9_part1(&input));
    println!("Day 9, part 2: {}", day9_part2(&input));

    let input =
        read_to_string("./input/day10.txt").expect("Should have been able to read input/day10.txt");
    println!("Day 10, part 1: {}", day10_part1(&input));
    println!("Day 10, part 2: {}", day10_part2(&input));

    let input =
        read_to_string("./input/day11.txt").expect("Should have been able to read input/day11.txt");
    println!("Day 11, part 1: {}", day11_part1(&input));
    println!("Day 11, part 2: {}", day11_part2(&input));

    let input =
        read_to_string("./input/day12.txt").expect("Should have been able to read input/day12.txt");
    println!("Day 12, part 1: {}", day12_part1(&input));
    println!("Day 12, part 2: {}", day12_part2(&input));

    let input =
        read_to_string("./input/day13.txt").expect("Should have been able to read input/day13.txt");
    println!("Day 13, part 1: {}", day13_part1(&input));
    println!("Day 13, part 2: {}", day13_part2(&input));

    let input =
        read_to_string("./input/day14.txt").expect("Should have been able to read input/day14.txt");
    println!("Day 14, part 1: {}", day14_part1(&input));
    println!("Day 14, part 2: {}", day14_part2(&input));

    let input =
        read_to_string("./input/day15.txt").expect("Should have been able to read input/day15.txt");
    println!("Day 15, part 1: {}", day15_part1(&input));
    println!("Day 15, part 2: {}", day15_part2(&input));

    let input =
        read_to_string("./input/day16.txt").expect("Should have been able to read input/day16.txt");
    println!("Day 16, part 1: {}", day16_part1(&input));
    println!("Day 16, part 2: {}", day16_part2(&input));

    let input =
        read_to_string("./input/day17.txt").expect("Should have been able to read input/day17.txt");
    println!("Day 17, part 1: {}", day17_part1(&input));
    println!("Day 17, part 2: {}", day17_part2(&input));

    let input =
        read_to_string("./input/day18.txt").expect("Should have been able to read input/day18.txt");
    println!("Day 18, part 1: {}", day18_part1(&input));
    println!("Day 18, part 2: {}", day18_part2(&input));

    let input =
        read_to_string("./input/day19.txt").expect("Should have been able to read input/day19.txt");
    println!("Day 19, part 1: {}", day19_part1(&input));
    println!("Day 19, part 2: {}", day19_part2(&input));

    let input =
        read_to_string("./input/day20.txt").expect("Should have been able to read input/day20.txt");
    println!("Day 20, part 1: {}", day20_part1(&input));
    println!("Day 20, part 2: {}", day20_part2(&input));

    let input =
        read_to_string("./input/day21.txt").expect("Should have been able to read input/day21.txt");
    println!("Day 21, part 1: {}", day21_part1(&input));
    println!("Day 21, part 2: {}", day21_part2(&input));

    let input =
        read_to_string("./input/day22.txt").expect("Should have been able to read input/day22.txt");
    println!("Day 22, part 1: {}", day22_part1(&input));
    println!("Day 22, part 2: {}", day22_part2(&input));

    let input =
        read_to_string("./input/day23.txt").expect("Should have been able to read input/day23.txt");
    println!("Day 23, part 1: {}", day23_part1(&input));
    println!("Day 23, part 2: {}", day23_part2(&input));

    let input =
        read_to_string("./input/day24.txt").expect("Should have been able to read input/day24.txt");
    println!("Day 24, part 1: {}", day24_part1(&input));
    println!("Day 24, part 2: {}", day24_part2(&input));

    let input =
        read_to_string("./input/day25.txt").expect("Should have been able to read input/day25.txt");
    println!("Day 25, part 1: {}", day25_part1(&input));
    println!("Day 25, part 2: {}", day25_part2(&input));
}
