use num::Integer;
use std::collections::HashSet;

// Part 1 note: this took me a long time. Most of the time wrestling with my rust knowledge. As
// with most of my solutions this feels overly verbose.

#[derive(Debug, PartialEq, Copy, Clone)]
enum MirrorDirection {
    Vertical,
    Horizontal,
}

#[derive(Debug, PartialEq, Copy, Clone)]
struct MirrorPoint {
    direction: MirrorDirection,
    count: u32,
}

fn is_horizontal_palindrome(input: &str) -> bool {
    if input.len().is_odd() {
        return false;
    }

    input.chars().zip(input.chars().rev()).all(|(a, b)| a == b)
}

fn is_vertical_palindrome(input: &str) -> bool {
    let line_count = input.lines().count();
    if line_count.is_odd() {
        return false;
    }

    input.lines().zip(input.lines().rev()).all(|(a, b)| a == b)
}

// This and vertical could probably be broken out to a trait? one acting on a single &str and the
// other acting on a Vec<&str>? 
// wait...and I don't need to test so much as I am :facepalm:. This just needs to find the one
// mirror point. Anyway, moving on.
fn count_above_horizontal_mirror(input: &str, ignoring: &Option<MirrorPoint>) -> Option<MirrorPoint> {
    let count = input.lines().count();
    let mut set: HashSet<i32> = HashSet::new();
    for i in (1..count).rev() {
        if is_vertical_palindrome(&input.lines().collect::<Vec<_>>()[0..i].join("\n")) {
            set.insert((i / 2) as i32);
        }
    }
    for i in 0..count-1 {
        if is_vertical_palindrome(&input.lines().collect::<Vec<_>>()[i..count].join("\n")) {
            set.insert(((i as i32 - count as i32))/2);
        }
    }

    if set.len() == 0 {
        return None;
    }

    let winner = set.iter().filter_map(|w| {
        let potential = if w.signum() == 1 { *w } else { count as i32 + *w };
        if ignoring.is_some_and(|i| i.count == (potential as u32)) {
            None
        } else {
            Some(potential as u32)
        }
    }).collect::<HashSet<u32>>();

    if winner.len() == 0 {
        return None;
    }
    assert_eq!(1, winner.len());

    Some(MirrorPoint {
        direction: MirrorDirection::Horizontal,
        count: *winner.iter().nth(0).unwrap(),
    })
}

fn count_left_of_vertical_mirror(input: &str, ignoring: &Option<MirrorPoint>) -> Option<MirrorPoint> {
    let col_count = input.lines().nth(0).unwrap().len();

    // look at each line and store the x position for each possible mirror:
    // look for palindrome from left edge, coming from the right.
    let winner = input
        .lines()
        .map(|line| {
            (1..col_count)
                .filter_map(|x| {
                    if is_horizontal_palindrome(line[0..x].as_ref()) {
                        Some((x / 2) as i32)
                    } else if is_horizontal_palindrome(line[x..col_count].as_ref()) {
                        let position = (x as i32 - col_count as i32)/2;
                        Some(position)
                    } else {
                        None
                    }
                })
                .collect::<HashSet<i32>>()
        })
        .reduce(|acc, item| &acc & &item);

    let winner = winner.unwrap();

    if winner.len() == 0 {
        return None;
    }

    let winner = winner.iter().filter_map(|w| {
        let potential = if w.signum() == 1 { *w } else { col_count as i32 + *w };
        if ignoring.is_some_and(|i| i.count == (potential as u32)) {
            None
        } else {
            Some(potential as u32)
        }
    }).collect::<HashSet<u32>>();

    if winner.len() == 0 {
        return None;
    }
    assert_eq!(1, winner.len());

    Some(MirrorPoint {
        direction: MirrorDirection::Vertical,
        count: *winner.iter().nth(0).unwrap(),
    })
}

pub fn day13_part1(input: &str) -> String {
    input
        .split("\n\n")
        .map(|map| {
            if let Some(left) = count_left_of_vertical_mirror(map, &None) {
                return left.count;
            } else if let Some(above) = count_above_horizontal_mirror(map, &None) {
                return above.count * 100;
            } else {
                unreachable!("should always have a mirror point?");
            }
        })
        .sum::<u32>()
        .to_string()
}

pub fn day13_part2(input: &str) -> String {
    // brute force hunt :grimacing:
    // this actually isn't slow[0] so :shrugging:
    //
    // [0] compared to yesterday and other days :sweatsmile:
    input
        .split("\n\n")
        .map(|map| {
            let original_left = count_left_of_vertical_mirror(map, &None);
            let original_above = count_above_horizontal_mirror(map, &None);

            let mut answer: Option<u32> = None;
            let mut idx: usize = 0;
            while answer.is_none() {
                let replacement;
                match map.get(idx..idx+1).unwrap() {
                    "." => replacement = "#",
                    "#" => replacement = ".",
                    _ => {idx+=1; continue;},
                }
                let mut test = map.to_string();
                test.replace_range(idx..idx+1, replacement);
                assert_ne!(map, test);
                let left = count_left_of_vertical_mirror(&test, &original_left);
                if left.is_some() && left != original_left {
                    answer = Some(left.unwrap().count);
                    break;
                }
                let above = count_above_horizontal_mirror(&test, &original_above);
                if above.is_some() {
                    answer = Some(above.unwrap().count * 100);
                    break;
                }
                idx+=1;
            }
            answer.unwrap()
        })
        .sum::<u32>()
        .to_string()
}

#[cfg(test)]
mod test {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("##..##", true)]
    #[case("##...##", false)]
    #[case("##.#..#.##", true)]
    #[case(".##..##.", true)]
    fn test_day13_is_horizontal_palindrome(#[case] input: &str, #[case] expected: bool) {
        assert_eq!(expected, is_horizontal_palindrome(input));
    }

    #[rstest]
    #[case("##..##
##..##", true)]
    #[case("##...##
##.#.##", false)]
    #[case("##.#
##.#
##.#", false)]
    #[case(".#.
#.#
#.#
.#.", true)]
    fn test_day13_is_vertical_palindrome(#[case] input: &str, #[case] expected: bool) {
        assert_eq!(expected, is_vertical_palindrome(input));
    }

    #[rstest]
    #[case(
        "##.#.##..##.
##.......#..",
       Some(MirrorPoint{direction: MirrorDirection::Vertical, count: 1})
    )]
    #[case(
        "...#........#..
##.#..#..#..#.#
##.##.####.##.#
##..##.##.##..#
...#..#..#..#..
..#..######..#.
...#.#.##.#.#..
..##...##...##.
...#...##...#..
##..###..###..#
...#..####..#..
...###....###..
.......##......
####..####..###
#####......###.
..#..######..#.
..#..#....#..#.",
       Some(MirrorPoint{direction: MirrorDirection::Vertical, count: 1})
    )]
    #[case(
        "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.",
       Some(MirrorPoint{direction: MirrorDirection::Vertical, count: 5}) 
    )]
    #[case(
        "#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#", None
    )]
    // #[case("##..#", 2)]
    // #[case("####..#", 2)]
    fn test_day13_count_left_of_vertical_mirror(#[case] input: &str, #[case] expected: Option<MirrorPoint>) {
        assert_eq!(expected, count_left_of_vertical_mirror(input, &None));
    }

#[rstest]
#[case(
"#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#
#...##..#",
        Some(MirrorPoint{direction: MirrorDirection::Horizontal, count: 3})
    )]
#[case(
"#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#",
        Some(MirrorPoint{direction: MirrorDirection::Horizontal, count: 4})
    )]
#[case(
    "#..##..
#.#.##.
#.####.
.####..
#.###.#
...####
#...#..
..#.#..
#...##.
.#####.
.#....#
.#####.
.####..
###.###
###.###
.####..
.#####.",
        Some(MirrorPoint{direction: MirrorDirection::Horizontal, count: 14})
    )]
    fn test_day13_count_above(#[case] input: &str, #[case] expected: Option<MirrorPoint>) {
        assert_eq!(expected, count_above_horizontal_mirror(input, &None));
    }

    #[rstest]
    #[case(
        "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#",
        "405"
    )]
    fn test_day13_part1(#[case] input: &str, #[case] expected: &str) {
        assert_eq!(expected, day13_part1(input));
    }
    #[rstest]
    #[case(
        "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#",
        "400"
    )]
    fn test_day13_part2(#[case] input: &str, #[case] expected: &str) {
        assert_eq!(expected, day13_part2(input));
    }
}
