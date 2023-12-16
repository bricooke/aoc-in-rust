struct Hasher {
    value: u32,
}

impl Hasher {
    // Determine the ASCII code for the current character of the string.
    // Increase the current value by the ASCII code you just determined.
    // Set the current value to itself multiplied by 17.
    // Set the current value to the remainder of dividing itself by 256.
    fn hash(&mut self, c: char) {
        let x = c as u32;
        self.value += x;
        self.value *= 17;
        self.value = self.value % 256;
    }
}

pub fn day15_part1(input: &str) -> String {
    input
        .replace("\n", "")
        .split(",")
        .map(|i| {
            let mut hasher = Hasher { value: 0 };
            for c in i.chars() {
                hasher.hash(c);
            }
            hasher.value
        })
        .sum::<u32>()
        .to_string()
}

pub fn day15_part2(_input: &str) -> String {
    todo!();
}

#[cfg(test)]
mod test {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("HASH", "52")]
    #[case("rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7", "1320")]
    fn test_day15_part1(#[case] input: &str, #[case] expected: &str) {
        assert_eq!(day15_part1(input), expected);
    }

    #[test]
    #[ignore]
    fn test_day15_part2() {}
}
