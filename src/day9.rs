use itertools::Itertools;

fn find_next_value(line: Vec<i32>) -> i32 {
    if line.iter().all(|i| *i == 0) {
        return 0;
    }

    let next_row = line
        .iter()
        .tuple_windows()
        .map(|(a, b)| b - a)
        .collect::<Vec<i32>>();

    line.last().unwrap() + find_next_value(next_row)
}

fn find_prev_value(line: Vec<i32>) -> i32 {
    if line.iter().all(|i| *i == 0) {
        return 0;
    }

    let next_row = line
        .iter()
        .tuple_windows()
        .map(|(a, b)| b - a)
        .collect::<Vec<i32>>();

    line.first().unwrap() - find_prev_value(next_row)
}

pub fn day9_part1(input: &str) -> String {
    let lines = input.lines().map(|line| {
        line.split(" ")
            .map(|v| v.parse::<i32>().unwrap())
            .collect::<Vec<i32>>()
    });

    lines
        .map(|line| find_next_value(line))
        .sum::<i32>()
        .to_string()
}

pub fn day9_part2(input: &str) -> String {
    let lines = input.lines().map(|line| {
        line.split(" ")
            .map(|v| v.parse::<i32>().unwrap())
            .collect::<Vec<i32>>()
    });

    lines
        .map(|line| find_prev_value(line))
        .sum::<i32>()
        .to_string()
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";

    #[test]
    fn test_day9_part1() {
        assert_eq!("114", day9_part1(INPUT));
    }

    #[test]
    fn test_day9_part2() {
        assert_eq!("2", day9_part2(INPUT));
    }
}
