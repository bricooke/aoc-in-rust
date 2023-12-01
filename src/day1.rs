fn words_to_digit_chars(line: &str) -> String {
    // This was silly. After writing this I scrolled through reddit and saw
    // suggestion to just replace "nine" with "n9ne", etc. :shrugging:
    let mut output = "".to_string();
    let mut current_word = "".to_string();
    for (i, _) in line.char_indices() {
        current_word.push_str(&line[i..=i]);
        let digit = if current_word.contains("one") {
            Some("1")
        } else if current_word.contains("two") {
            Some("2")
        } else if current_word.contains("three") {
            Some("3")
        } else if current_word.contains("four") {
            Some("4")
        } else if current_word.contains("five") {
            Some("5")
        } else if current_word.contains("six") {
            Some("6")
        } else if current_word.contains("seven") {
            Some("7")
        } else if current_word.contains("eight") {
            Some("8")
        } else if current_word.contains("nine") {
            Some("9")
        } else {
            None
        };
        if digit.is_some() {
            current_word.remove(current_word.len() - 2);
            output.push_str(digit.unwrap());
        }
        output.push_str(&line[i..=i]);
    }
    output
}

pub fn day1_part1(input: &str) -> String {
    let numbers = '0'..='9';
    let temp = input.lines().map(|line| {
        let digits = line
            .chars()
            .filter_map(|c| if numbers.contains(&c) { Some(c) } else { None })
            .collect::<Vec<_>>();
        let digit = digits[0].to_string() + digits.last().unwrap().to_string().as_str();
        digit.parse::<u32>().unwrap()
    });

    let _ = temp
        .clone()
        .inspect(|f| {
            dbg!(f);
        })
        .collect::<Vec<_>>();

    temp.sum::<u32>().to_string()
}

pub fn day1_part2(input: &str) -> String {
    let input = input
        .lines()
        .map(|line| words_to_digit_chars(line))
        .collect::<Vec<_>>()
        .join("\n");
    day1_part1(input.as_str())
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";

    #[test]
    fn test_day1_part1() {
        assert_eq!("142", day1_part1(INPUT));
    }

    #[test]
    fn test_day1_part2() {
        assert_eq!("37", day1_part2("threetwo1nine7"));
        assert_eq!("142", day1_part2(INPUT));
        assert_eq!("29", day1_part2("two1nine"));
        assert_eq!("37", day1_part2("threetwo1ninzeroe7z"));
        assert_eq!("33", day1_part2("3"));
        assert_eq!("33", day1_part2("three3"));
        assert_eq!("33", day1_part2("three4fourfiveseventhree"));
        assert_eq!("59", day1_part2("fiverfourfivefive1three9"));
        assert_eq!("99", day1_part2("ninexhskkhdkgjgvjhrqhrfj9bnrfbtxpp"));
        assert_eq!("11", day1_part2("oneone"));
        assert_eq!("22", day1_part2("twotwo"));
        assert_eq!("33", day1_part2("three"));
        assert_eq!("79", day1_part2("sevenine"));
        assert_eq!("72", day1_part2("ssevenhcltwoseven2cxrmxxcr"));

        assert_eq!(
            "281",
            day1_part2(
                "two1nine
eighthree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen"
            )
        );
    }

    #[test]
    fn test_words_to_digits() {
        assert_eq!("seve7nin9e", words_to_digit_chars("sevenine"));
        assert_eq!("eigh8tw2othre3e", words_to_digit_chars("eightwothree"));
        assert_eq!(
            "sseve7nhcltw2oseve7n2cxrmxxcr",
            words_to_digit_chars("ssevenhcltwoseven2cxrmxxcr")
        );
        assert_eq!("eigh8tw2othre3e4", words_to_digit_chars("eightwothree4"));
    }
}
