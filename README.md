Simple rust project structure for [Advent of Code](https://adventofcode.com).

There's nothing fancy going on here. I was just going through 2022 and realized it would be easy to hand make a template to save some of the boiler plate setup for each day.

Each day:
1. copy your input in to `input/day{x}.txt`
1. Edit `day{x}.rs`:
    1. Set `INPUT` to be the sample input.
    1. Update the `assert_eq!` lines to match the example provided
    1. Implement the `day{x}_part1` and `day{x}_part2` functions
1. `cargo test` to run your tests with the example input
1. `cargo run` to run with your input

