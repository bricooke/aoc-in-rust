use itertools::Itertools;
use std::collections::HashMap;

// credit hyperneutrino https://www.youtube.com/watch?v=g3Ms5e7Jdqo
// I brute forced part 1 :sweatsmile: but wouldn't have gotten part2 with that.
fn count_springs<'a>(
    springs: &'a str,
    pattern: &[usize],
    cache: &mut HashMap<(&'a str, Vec<usize>), usize>,
) -> usize {
    if springs.is_empty() {
        // 1 if the pattern is also empty (this was valid)
        return if pattern.is_empty() { 1 } else { 0 };
    }

    if pattern.is_empty() {
        // 1 only if springs only has . or ? (all the ?'s would become .'s for a volid match)
        return if springs.contains('#') { 0 } else { 1 };
    }

    let key = (springs, pattern.to_vec());
    if cache.get(&key).is_some() {
        return *cache.get(&key).unwrap();
    }

    let mut result = 0;

    let first_spring = springs.chars().nth(0).unwrap();
    match first_spring {
        '.' | '?' => {
            // recurse without the leading spring char, treating it as a .
            result += count_springs(&springs[1..], pattern, cache);
        }
        _ => (),
    }

    match first_spring {
        '#' | '?' => {
            if pattern[0] <= springs.len() // only consider if the springs count can equal needed pattern amount
                && !springs[0..pattern[0]].contains('.') // and that number of springs are operational
                && (pattern[0] == springs.len() || springs.chars().nth(pattern[0]).unwrap() != '#')
            // ^ and (springs can all be springs to satisfy the pattern || the spring at the end
            // of the pattern can be broken (to satisfy the border needed)
            {
                let springs = springs.get(pattern[0] + 1..).or(Some("")).unwrap();
                result += count_springs(&springs, &pattern[1..], cache);
            }
        }
        _ => (),
    }
    cache.insert(key, result);
    result
}

pub fn day12_part1(input: &str) -> String {
    let mut cache: HashMap<(&str, Vec<usize>), usize> = HashMap::new();
    let mut pattern: Vec<usize> = Vec::new();
    input
        .lines()
        .map(|line| {
            let mut split = line.split(" ");
            let spring_sheet = split.next().unwrap();
            pattern = split
                .next()
                .unwrap()
                .split(",")
                .map(|p| p.parse::<usize>().unwrap())
                .collect();

            count_springs(spring_sheet, &pattern, &mut cache)
        })
        .sum::<usize>()
        .to_string()
}

pub fn day12_part2(input: &str) -> String {
    let input = input
        .lines()
        .map(|line| {
            let sides = line.split(" ").collect::<Vec<&str>>();
            let springs = sides.first().unwrap();
            let count_springss = sides.last().unwrap();
            let springs = (0..5).map(|_| springs).join("?");
            let count_springss = (0..5).map(|_| *count_springss).join(",");
            vec![springs, count_springss].join(" ")
        })
        .collect::<Vec<_>>()
        .join("\n");
    day12_part1(&input)
}

#[cfg(test)]
mod test {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(
        "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1",
        "21"
    )]
    fn test_day12_part1(#[case] input: &str, #[case] expected: &str) {
        assert_eq!(expected, day12_part1(input));
    }

    #[rstest]
    #[case(
        "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1",
        "525152"
    )]
    fn test_day12_part2(#[case] input: &str, #[case] expected: &str) {
        assert_eq!(expected, day12_part2(input));
    }
}
