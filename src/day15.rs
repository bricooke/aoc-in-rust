use std::collections::{HashMap, VecDeque};

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

    fn hash_str(&mut self, str: &str) {
        for c in str.chars() {
            self.hash(c);
        }
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

#[derive(Debug)]
enum Command<'a> {
    Set((&'a str, u8)),
    Remove(&'a str),
}

pub fn day15_part2(input: &str) -> String {
    // Hash of box id to sorted VecDeque of tuples with the label + the lens focal length
    let mut boxes: HashMap<u8, VecDeque<(&str, u8)>> = HashMap::new();
    let input = input.replace("\n", "");
    input.split(",").for_each(|cmd| {
        let command = if cmd.contains("=") {
            let mut cmd = cmd.split("=");
            Command::Set((
                cmd.nth(0).unwrap(),
                cmd.nth(0).unwrap().parse::<u8>().unwrap(),
            ))
        } else if cmd.contains("-") {
            Command::Remove(cmd.split("-").nth(0).unwrap())
        } else {
            unreachable!();
        };

        let mut hasher = Hasher { value: 0 };
        match command {
            Command::Set((a, _)) => hasher.hash_str(a),
            Command::Remove(a) => hasher.hash_str(a),
        }

        let box_id = hasher.value as u8;
        if !boxes.contains_key(&box_id) {
            boxes.insert(box_id, VecDeque::new());
        }
        let box_contents = boxes.get_mut(&box_id).unwrap();
        let existing = box_contents.iter().enumerate().find(|(_, v)| {
            v.0 == match command {
                Command::Set(cmd) => cmd.0,
                Command::Remove(cmd) => cmd,
            }
        });

        match command {
            Command::Set((label, focal_length)) => {
                if existing.is_some() {
                    let position = existing.unwrap().0;
                    box_contents.remove(position);
                    box_contents.insert(position, (label, focal_length));
                } else {
                    box_contents.push_back((label, focal_length));
                }
            }
            Command::Remove(_) => {
                if existing.is_some() {
                    box_contents.remove(existing.unwrap().0);
                }
            }
        }
    });

    // One plus the box number of the lens in question.
    // The slot number of the lens within the box: 1 for the first lens, 2 for the second lens, and so on.
    // The focal length of the lens.

    boxes
        .iter()
        .flat_map(|(box_id, cabinet)| {
            cabinet.iter().enumerate().map(|(slot, slot_content)| {
                (*box_id as usize + 1) * (slot as usize + 1) * slot_content.1 as usize
            })
        })
        .sum::<usize>()
        .to_string()
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

    #[rstest]
    #[case("rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7", "145")]
    fn test_day15_part2(#[case] input: &str, #[case] expected: &str) {
        assert_eq!(day15_part2(input), expected);
    }
}
