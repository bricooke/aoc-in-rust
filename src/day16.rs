use std::{
    collections::{HashMap, HashSet},
    fmt::Display,
};

use glam::IVec2;

#[derive(Debug, PartialEq, Clone, Copy, Eq, Hash)]
enum Direction {
    North,
    South,
    East,
    West,
}

#[derive(Debug, PartialEq, Clone, Copy, Eq, Hash)]
struct Beam {
    position: IVec2,
    direction: Direction,
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum Piece {
    VerticalSplitter,   // |
    HorizontalSplitter, // -
    UpwardMirror,       // /
    DownwardMirror,     // \
}

#[derive(Debug, PartialEq, Clone)]
struct Cave {
    beams: Vec<Beam>,
    map: HashMap<IVec2, Piece>,
    bounds: IVec2,
    energized: HashSet<IVec2>,
    moves_without_energizing: u32,
    cache: HashMap<Vec<Beam>, u32>,
}

impl Display for Piece {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                VerticalSplitter => '|',
                HorizontalSplitter => '-',
                UpwardMirror => '/',
                DownwardMirror => '\\',
            }
        )
    }
}

impl Display for Cave {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.bounds.y {
            for x in 0..self.bounds.x {
                let position = IVec2 { x, y };
                if self.energized.contains(&position) {
                    let _ = write!(f, "#");
                    continue;
                }
                let _ = match self.map.get(&position) {
                    None => write!(f, "."),
                    Some(piece) => write!(f, "{}", piece),
                };
            }
            let _ = write!(f, "\n");
        }
        write!(f, "")
    }
}

use Direction::*;
use Piece::*;

fn build_cave(input: &str) -> Cave {
    let max_x = input.lines().nth(0).unwrap().chars().count() as i32;
    let max_y = input.lines().count() as i32;

    let map = input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars().enumerate().filter_map(move |(x, c)| {
                let piece = match c {
                    '|' => Some(VerticalSplitter),
                    '-' => Some(HorizontalSplitter),
                    '/' => Some(UpwardMirror),
                    '\\' => Some(DownwardMirror),
                    _ => None,
                };
                if let Some(piece) = piece {
                    Some((
                        IVec2 {
                            x: x as i32,
                            y: y as i32,
                        },
                        piece,
                    ))
                } else {
                    None
                }
            })
        })
        .collect::<HashMap<IVec2, Piece>>();

    let mut beams = Vec::new();
    beams.push(Beam {
        direction: East,
        position: IVec2 { x: -1, y: 0 },
    });

    Cave {
        map,
        beams,
        bounds: IVec2 { x: max_x, y: max_y },
        energized: HashSet::new(),
        moves_without_energizing: 0,
        cache: HashMap::new(),
    }
}

impl Cave {
    fn advance_light(&mut self) -> bool {
        let originally_energized_count = self.energized.len();
        let mut new_beams = Vec::new();
        self.beams.iter().for_each(|beam| {
            let current_position = beam.position;
            let next_position = current_position
                + match beam.direction {
                    North => IVec2 { x: 0, y: -1 },
                    South => IVec2 { x: 0, y: 1 },
                    East => IVec2 { x: 1, y: 0 },
                    West => IVec2 { x: -1, y: 0 },
                };

            // are we off the board?
            if next_position.x < 0
                || next_position.y < 0
                || next_position.y >= self.bounds.y
                || next_position.x >= self.bounds.x
            {
                return;
            }

            // Figure out what the next position's piece does to us.
            match self.map.get(&next_position) {
                Some(VerticalSplitter) => match beam.direction {
                    North | South => {
                        new_beams.push(Beam {
                            position: next_position,
                            direction: beam.direction,
                        });
                    }
                    East | West => {
                        new_beams.push(Beam {
                            position: next_position,
                            direction: North,
                        });
                        new_beams.push(Beam {
                            position: next_position,
                            direction: South,
                        });
                    }
                },
                Some(HorizontalSplitter) => match beam.direction {
                    East | West => {
                        new_beams.push(Beam {
                            position: next_position,
                            direction: beam.direction,
                        });
                    }
                    North | South => {
                        new_beams.push(Beam {
                            position: next_position,
                            direction: West,
                        });
                        new_beams.push(Beam {
                            position: next_position,
                            direction: East,
                        });
                    }
                },
                Some(UpwardMirror) => {
                    let next_direction = match beam.direction {
                        North => East,
                        South => West,
                        East => North,
                        West => South,
                    };
                    new_beams.push(Beam {
                        position: next_position,
                        direction: next_direction,
                    });
                }
                Some(DownwardMirror) => {
                    let next_direction = match beam.direction {
                        North => West,
                        South => East,
                        East => South,
                        West => North,
                    };
                    new_beams.push(Beam {
                        position: next_position,
                        direction: next_direction,
                    });
                }
                None => new_beams.push(Beam {
                    position: next_position,
                    direction: beam.direction,
                }),
            }

            self.energized.insert(next_position);
        });

        self.beams = new_beams;
        if originally_energized_count == self.energized.len() {
            self.moves_without_energizing += 1;
        } else {
            self.moves_without_energizing = 0;
        }

        // / 2 to speed it up a tiny bit and this is enough to cover things...:science:
        self.beams.len() > 0 && (self.moves_without_energizing as i32) < (self.bounds.x / 2)
    }
}

pub fn day16_part1(input: &str) -> String {
    let mut cave = build_cave(input);

    while cave.advance_light() {}

    cave.energized.len().to_string()
}

pub fn day16_part2(input: &str) -> String {
    // brute force? then memoize?
    let mut cave = build_cave(input);
    let mut winning = 0;

    for y in 0..cave.bounds.y {
        for x in vec![-1, cave.bounds.x] {
            cave.beams = vec![];
            cave.energized = HashSet::new();

            cave.beams.push(Beam {
                position: IVec2 { x, y },
                direction: if x < 0 { East } else { West },
            });

            while cave.advance_light() {}

            winning = winning.max(cave.energized.len());
        }
    }

    for x in 0..cave.bounds.x {
        for y in vec![-1, cave.bounds.y] {
            cave.beams = vec![];
            cave.energized = HashSet::new();

            cave.beams.push(Beam {
                position: IVec2 { x, y },
                direction: if y < 0 { South } else { North },
            });

            while cave.advance_light() {}

            winning = winning.max(cave.energized.len());
        }
    }

    winning.to_string()
}

#[cfg(test)]
mod test {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(
        r#".|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|...."#,
        "46"
    )]
    fn test_day16_part1(#[case] input: &str, #[case] expected: &str) {
        assert_eq!(day16_part1(input), expected);
    }

    #[rstest]
    #[case(
        r#".|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|...."#,
        "51"
    )]
    fn test_day16_part2(#[case] input: &str, #[case] expected: &str) {
        assert_eq!(day16_part2(input), expected);
    }
}
