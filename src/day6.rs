use itertools::Itertools;

#[derive(Debug, PartialEq)]
struct Race {
    time: u64,
    distance: u64,
}
fn races(input: &str) -> Vec<Race> {
    input
        .trim()
        .lines()
        .chunks(2)
        .into_iter()
        .map(|lines| {
            let lines = lines.collect_vec();
            let times = lines[0];
            let distances = lines[1];

            let times = times
                .split(":")
                .last()
                .unwrap()
                .trim()
                .split(" ")
                .filter_map(|t| t.trim().parse::<u64>().ok());
            let distances = distances
                .split(":")
                .last()
                .unwrap()
                .trim()
                .split(" ")
                .filter_map(|t| t.trim().parse::<u64>().ok());
            times
                .zip(distances)
                .map(|(time, distance)| Race { time, distance })
                .collect_vec()
        })
        .flatten()
        .collect::<Vec<Race>>()
}

impl Race {
    fn winning_hold_times(&self) -> u64 {
        let mut winners: Vec<u64> = Vec::new();

        for time in 1..self.time {
            let time_to_travel = self.time - time;
            let speed = time;
            let distance_traveled = speed * time_to_travel;
            if distance_traveled > self.distance {
                winners.push(time)
            }
        }
        winners.len() as u64
    }
}

pub fn day6_part1(input: &str) -> String {
    let races = races(input);
    races
        .iter()
        .map(|race| race.winning_hold_times())
        .product::<u64>()
        .to_string()
}

pub fn day6_part2(input: &str) -> String {
    let input = input
        .lines()
        .map(|line| {
            let line = line
                .split(":")
                .last()
                .unwrap()
                .chars()
                .filter(|c| *c != ' ')
                .collect::<String>();
            "Blah: ".to_string() + &line + "\n"
        })
        .collect::<String>();
    day6_part1(&input)
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = "Time:      7  15   30
Distance:  9  40  200";

    #[test]
    fn test_parser() {
        assert_eq!(
            vec![
                Race {
                    time: 7,
                    distance: 9
                },
                Race {
                    time: 15,
                    distance: 40
                },
                Race {
                    time: 30,
                    distance: 200
                }
            ],
            races(INPUT)
        );
    }

    #[test]
    fn test_parser_p2() {
        assert_eq!(
            vec![Race {
                time: 47707566,
                distance: 282107911471062
            },],
            races(
                "
Blah: 47707566
Blah: 282107911471062
"
            )
        );
    }

    #[test]
    fn test_day6_part1() {
        assert_eq!("288", day6_part1(INPUT));
    }

    #[test]
    fn test_day6_part2() {
        assert_eq!("71503", day6_part2(INPUT));
    }
}
