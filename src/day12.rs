use itertools::{repeat_n, Itertools};
use nom::{
    branch::alt,
    bytes::complete::{tag, take_till},
    character::complete::{self},
    multi::{many1, separated_list0, separated_list1},
    IResult, Parser,
};
use nom_locate::LocatedSpan;
type Span<'a> = LocatedSpan<&'a str>;

pub fn day12_part1(input: &str) -> String {
    input
        .lines()
        .map(|line| arrangements(line))
        .sum::<u32>()
        .to_string()
}

fn expand(line: &str) -> String {
    println!("Expanding 1...");
    let sides = line.split(" ").collect::<Vec<&str>>();
    let springs = sides.first().unwrap();
    let counts = sides.last().unwrap();

    let springs = (0..5).map(|_| springs.clone()).join("?");
    let counts = (0..5).map(|_| (*counts).clone()).join(",");
    vec![springs, counts].join(" ")
}

pub fn day12_part2(input: &str) -> String {
    input
        .lines()
        .map(|line| expand(line))
        .inspect(|line| {
            dbg!(line);
        })
        .map(|line| arrangements(&line))
        .sum::<u32>()
        .to_string()
}

fn is_valid<'a>(arrangement: &'a str, counts: &Vec<u32>) -> bool {
    let iresult: IResult<&str, &str> = take_till(|c| c == '#')(arrangement);
    let input = iresult.unwrap().0;

    let iresult: IResult<&str, Vec<Vec<char>>> =
        separated_list0(many1(tag(".")), many1(complete::char('#')))(input);
    let (_, pattern) = iresult.expect("should have been valid input");
    if pattern.len() != counts.len() {
        return false;
    }

    let invalid_exists = pattern
        .iter()
        .zip(counts.iter())
        .find(|(p, c)| p.len() != **c as usize);
    if invalid_exists.is_some() {
        false
    } else {
        true
    }
}

fn arrangements(input: &str) -> u32 {
    println!("arrangements 1...");
    let sides = input.split(" ").collect::<Vec<&str>>();
    let springs = sides.first().unwrap();
    let counts = sides.last().unwrap();
    let counts = counts
        .split(",")
        .map(|c| c.parse::<u32>().unwrap())
        .collect::<Vec<_>>();

    // when in doubt, brute force it out? :thinking:
    // find all blocks of ??? and turn them in to all permutions and pass each in to the is_valid
    // check
    let iresult: IResult<Span, Span> = take_till(|c| c == '?')(Span::new(springs));
    let input = iresult.expect("parsing should succeed").0;
    let iresult: IResult<Span, Vec<(usize, usize)>> = separated_list1(
        many1(alt((tag("."), tag("#")))),
        many1(tag("?")).map(|span| {
            let info: Span = *span.first().unwrap();
            (info.get_column() - 1, span.len())
        }),
    )(input);
    let (_, unknowns) = iresult.expect("parsing failed");

    // We now have:
    // counts as a Vec
    // unknowns as (col, len)
    //
    // now generate all permutations for the unknown entries and see how many are valid
    // repeat_n(vec!['.', '#'].iter(), UNKNOWN LENGTH).multi_cartesian_product().collect::<Vec<_>>()
    let line = springs.to_string();
    let unknown_counts = unknowns.iter().map(|(_, len)| len).sum::<usize>();
    let samples = vec!['.', '#'];

    let mut arrangements = repeat_n(samples.iter(), unknown_counts)
        .multi_cartesian_product()
        .map(|pattern| pattern.into_iter().collect::<String>())
        .collect::<Vec<_>>();

    // Now take each of those and map them over the actual spring layout.
    let arrangements = arrangements
        .iter_mut()
        .map(|a| {
            line.chars()
                .into_iter()
                .map(|c| if c == '?' { a.pop().unwrap() } else { c })
                .collect::<String>()
        })
        .collect::<Vec<_>>();
    arrangements
        .iter()
        .filter(|a| is_valid(a, &counts))
        .collect::<Vec<_>>()
        .len() as u32
}

#[cfg(test)]
mod test {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(".#...#....###.", vec![1,1,3], true)]
    fn test_day12_part1_is_valid(
        #[case] input: &str,
        #[case] counts: Vec<u32>,
        #[case] expected: bool,
    ) {
        assert_eq!(expected, is_valid(input, &counts));
    }

    #[rstest]
    #[case("???.### 1,1,3", 1)]
    #[case(".??..??...?##. 1,1,3", 4)]
    fn test_day12_part1_arrangements(#[case] input: &str, #[case] expected: u32) {
        assert_eq!(expected, arrangements(input));
    }

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

    #[rstest]
    #[case(".# 1", ".#?.#?.#?.#?.# 1,1,1,1,1")]
    #[case(
        "???.### 1,1,3",
        "???.###????.###????.###????.###????.### 1,1,3,1,1,3,1,1,3,1,1,3,1,1,3"
    )]
    fn test_day12_expand(#[case] input: &str, #[case] expected: &str) {
        assert_eq!(expected, expand(input));
    }
}
