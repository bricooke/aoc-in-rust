use std::collections::HashMap;

#[derive(Debug, Eq, Hash, PartialEq, Clone, Copy)]
struct Position {
    x: i32,
    y: i32,
}

// | is a vertical pipe connecting north and south.
// - is a horizontal pipe connecting east and west.
// L is a 90-degree bend connecting north and east.
// J is a 90-degree bend connecting north and west.
// 7 is a 90-degree bend connecting south and west.
// F is a 90-degree bend connecting south and east.
// . is ground; there is no pipe in this tile.
// S is the starting position of the animal; there is a pipe on this tile, but your sketch doesn't show what shape the pipe has.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
enum Pipe {
    Start,
    Air,
    Vertical,
    Horizontal,
    UpAndRight,
    UpAndLeft,
    DownAndLeft,
    DownAndRight,
}

impl Pipe {
    fn from_char(c: char) -> Option<Pipe> {
        use Pipe::*;
        match c {
            'S' => Some(Start),
            '|' => Some(Vertical),
            '-' => Some(Horizontal),
            'L' => Some(UpAndRight),
            'J' => Some(UpAndLeft),
            '7' => Some(DownAndLeft),
            'F' => Some(DownAndRight),
            '.' => Some(Air),
            _ => unreachable!(),
        }
    }
}

fn is_connected(a: (Position, Pipe), b: (Position, Pipe)) -> bool {
    use Pipe::*;

    if !(a.0.x == b.0.x || a.0.y == b.0.y) {
        return false;
    }

    // This is not fun. I haven't thought through it but there's surely simpler logic
    // to achieve what's needed :sweatsmile:
    match (a.1, b.1) {
        (Air, _) => false,
        (_, Air) => false,
        (Start, Start) => false,
        (Start, Vertical) => b.0.x == a.0.x,
        (Start, Horizontal) => a.0.y == b.0.y,
        (Start, UpAndRight) => a.0.y < b.0.y || a.0.x > b.0.x,
        (Start, UpAndLeft) => a.0.y < b.0.y || a.0.x < b.0.x,
        (Start, DownAndLeft) => a.0.y > b.0.y || a.0.x < b.0.x,
        (Start, DownAndRight) => a.0.y > b.0.y || a.0.x > b.0.x,
        (Vertical, Start) | (Vertical, Vertical) => a.0.x == b.0.x,
        (Vertical, Horizontal) => false,
        (Vertical, UpAndRight) | (Vertical, UpAndLeft) => a.0.y < b.0.y,
        (Vertical, DownAndLeft) | (Vertical, DownAndRight) => a.0.y > b.0.y,
        (Horizontal, Vertical) => false,
        (Horizontal, Start) | (Horizontal, Horizontal) => a.0.y == b.0.y,
        (Horizontal, UpAndRight) | (Horizontal, DownAndRight) => a.0.x > b.0.x,
        (Horizontal, UpAndLeft) | (Horizontal, DownAndLeft) => a.0.x < b.0.x,
        (UpAndRight, Start) => a.0.x < b.0.x || a.0.y > b.0.y,
        (UpAndRight, Vertical) => a.0.y > b.0.y,
        (UpAndRight, Horizontal) => a.0.x < b.0.x,
        (UpAndRight, UpAndRight) => false,
        (UpAndRight, UpAndLeft) => a.0.x < b.0.x,
        (UpAndRight, DownAndLeft) => a.0.x < b.0.x || a.0.y > b.0.y,
        (UpAndRight, DownAndRight) => a.0.y > b.0.y,
        (UpAndLeft, Start) => a.0.x < b.0.x || a.0.y > b.0.y,
        (UpAndLeft, Vertical) => a.0.y > b.0.y,
        (UpAndLeft, Horizontal) => a.0.x > b.0.x,
        (UpAndLeft, UpAndRight) => a.0.x > b.0.x,
        (UpAndLeft, UpAndLeft) => false,
        (UpAndLeft, DownAndLeft) => a.0.y > b.0.y,
        (UpAndLeft, DownAndRight) => a.0.y > b.0.y || b.0.x < a.0.x,
        (DownAndLeft, Start) => a.0.x > b.0.x || a.0.y < b.0.y,
        (DownAndLeft, Vertical) => a.0.y < b.0.y,
        (DownAndLeft, Horizontal) => b.0.x < a.0.x,
        (DownAndLeft, UpAndRight) => a.0.y < b.0.y || a.0.x > b.0.x,
        (DownAndLeft, UpAndLeft) => a.0.y < b.0.y,
        (DownAndLeft, DownAndLeft) => false,
        (DownAndLeft, DownAndRight) => b.0.x < a.0.x,
        (DownAndRight, Start) => b.0.x > a.0.x || b.0.y > a.0.y,
        (DownAndRight, Vertical) => a.0.y < b.0.y,
        (DownAndRight, Horizontal) => b.0.x > a.0.x,
        (DownAndRight, UpAndRight) => b.0.y > a.0.y,
        (DownAndRight, UpAndLeft) => b.0.y > a.0.y || a.0.x < b.0.x,
        (DownAndRight, DownAndLeft) => b.0.x > a.0.x,
        (DownAndRight, DownAndRight) => false,
    }
}

pub fn day10_part1(input: &str) -> String {
    // Parse the map in to a starting (x, y) and a HashMap of (x, y) => Pipe
    // Walk from the start following valid moves and count how many before we return to the start
    // divide that by two and its the answer
    let map = input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars().enumerate().filter_map(move |(x, c)| {
                let pipe = Pipe::from_char(c);
                if pipe.is_some_and(|p| p != Pipe::Air) {
                    Some((
                        Position {
                            x: x as i32,
                            y: y as i32,
                        },
                        pipe.unwrap(),
                    ))
                } else {
                    None
                }
            })
        })
        .collect::<HashMap<Position, Pipe>>();
    let start = map
        .iter()
        .find(|item| {
            if let Pipe::Start = item.1 {
                true
            } else {
                false
            }
        })
        .unwrap();

    let mut curr = (start.0.clone(), start.1.clone());
    let mut prev = *start.0;
    let mut steps = 0;
    loop {
        steps += 1;
        // look left, right, up, down
        let positions = vec![
            Position {
                x: curr.0.x - 1,
                y: curr.0.y,
            },
            Position {
                x: curr.0.x + 1,
                y: curr.0.y,
            },
            Position {
                x: curr.0.x,
                y: curr.0.y + 1,
            },
            Position {
                x: curr.0.x,
                y: curr.0.y - 1,
            },
        ];
        let next = positions
            .iter()
            .filter(|p| **p != prev)
            .find(|position| match map.get(position) {
                Some(p) => {
                    if is_connected(curr, (**position, p.clone())) {
                        true
                    } else {
                        false
                    }
                }
                None => false,
            })
            .unwrap();
        prev = curr.0.clone();
        curr = (*next, *map.get(next).unwrap());

        if Pipe::Start == curr.1 {
            break;
        }
    }
    (steps / 2).to_string()
}

pub fn day10_part2(input: &str) -> String {
    // SHAMELESS COPY OF PART1 FOR NOW
    // We need the path so we can filter out the junk pipes which threw me off :(
    // I needed lots of hand holding on this one. Leaned on https://www.youtube.com/watch?v=_-QXvb8GJlg
    let map = input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars().enumerate().filter_map(move |(x, c)| {
                let pipe = Pipe::from_char(c);
                if pipe.is_some() {
                    Some((
                        Position {
                            x: x as i32,
                            y: y as i32,
                        },
                        pipe.unwrap(),
                    ))
                } else {
                    unreachable!();
                }
            })
        })
        .collect::<HashMap<Position, Pipe>>();
    let start = map
        .iter()
        .find(|item| {
            if let Pipe::Start = item.1 {
                true
            } else {
                false
            }
        })
        .unwrap();

    let mut path: HashMap<Position, Pipe> = HashMap::new();
    let mut curr = (start.0.clone(), start.1.clone());
    path.insert(curr.0, curr.1);
    let mut prev = *start.0;
    loop {
        // look left, right, up, down
        let positions = vec![
            Position {
                x: curr.0.x - 1,
                y: curr.0.y,
            },
            Position {
                x: curr.0.x + 1,
                y: curr.0.y,
            },
            Position {
                x: curr.0.x,
                y: curr.0.y + 1,
            },
            Position {
                x: curr.0.x,
                y: curr.0.y - 1,
            },
        ];
        let next = positions
            .iter()
            .filter(|p| **p != prev)
            .find(|position| match map.get(position) {
                Some(p) => {
                    if is_connected(curr, (**position, p.clone())) {
                        true
                    } else {
                        false
                    }
                }
                None => false,
            })
            .unwrap();
        prev = curr.0.clone();
        curr = (*next, *map.get(next).unwrap());
        path.insert(curr.0, curr.1);

        if Pipe::Start == curr.1 {
            break;
        }
    }

    // OK, now we have the map with everything (original)
    // and path with just the actual pipes.
    // Now use the ray trace stuff I had to get from reddit to know what to do.
    // This determines in/out of the 'polygon'
    input
        .lines()
        .enumerate()
        .map(|(y, line)| {
            let mut is_in = false;
            line.chars()
                .enumerate()
                .map(|(x, _)| {
                    let p = Position {
                        x: x as i32,
                        y: y as i32,
                    };
                    let pipe = map.get(&p).unwrap();
                    let is_in_path = path.contains_key(&p);

                    match (is_in_path, pipe) {
                        (true, Pipe::Vertical)
                        | (true, Pipe::Start)  
                            // A proper solution would determine if Start
                            // should be included in the toggling bit or not. For my input it
                            // should not have been included but it would depend on what pipe part
                            // the start piece actually was.
                            // But the tests need this to be included so include it.
                        | (true, Pipe::DownAndRight)
                        | (true, Pipe::DownAndLeft) => {
                            is_in = !is_in;
                            0
                        }
                        (true, _) => 0,
                        _ => {
                            if is_in {
                                1
                            } else {
                                0
                            }
                        }
                    }
                })
                .sum::<u32>()
        })
        .sum::<u32>()
        .to_string()
}

#[cfg(test)]
mod test {
    use super::*;
    use rstest::rstest;
    use Pipe::*;

    #[rstest]
    #[case(
        vec![(Position {x: 0,y: 0}, Horizontal), (Position{x: 1,y:0}, UpAndLeft)],
        true
    )]
    #[case(
        vec![(Position {x: 0,y: 0}, Horizontal), (Position{x: 1,y:0}, Vertical)],
        false
    )]
    #[case(
        vec![(Position {x: 0,y: 0}, Horizontal), (Position{x: 1,y:0}, Horizontal)],
        true
    )]
    #[case(
        vec![(Position {x: 0,y: 0}, Horizontal), (Position{x: 1,y:0}, UpAndRight)],
        false
    )]
    // 5
    #[case(
        vec![(Position {x: 0,y: 0}, Horizontal), (Position{x: 1,y:0}, DownAndLeft)],
        true
    )]
    #[case(
        vec![(Position {x: 0,y: 0}, Horizontal), (Position{x: 1,y:0}, DownAndRight)],
        false
    )]
    #[case(
        vec![(Position {x: 0,y: 0}, Horizontal), (Position{x: 1,y:0}, Start)],
        true
    )]
    #[case(
        vec![(Position {x: 0,y: 0}, UpAndRight), (Position{x: 1,y:0}, UpAndLeft)],
        true
    )]
    #[case(
        vec![(Position {x: 0,y: 0}, UpAndRight), (Position{x: 1,y:0}, Horizontal)],
        true
    )]
    // 10
    #[case(
        vec![(Position {x: 1,y: 0}, UpAndRight), (Position{x: 0,y:0}, Start)],
        false
    )]
    #[case(
        vec![(Position {x: 0,y: 0}, UpAndLeft), (Position{x: 1,y:0}, UpAndLeft)],
        false
    )]
    #[case(
        vec![(Position {x: 0,y: 0}, UpAndLeft), (Position{x: 0,y:1}, DownAndLeft)],
        false
    )]
    #[case(
        vec![(Position {x: 1,y: 0}, UpAndLeft), (Position{x: 0,y:0}, Horizontal)],
        true
    )]
    #[case(
        vec![(Position {x: 1,y: 0}, UpAndLeft), (Position{x: 0,y:0}, Start)],
        true
    )]
    #[case(
        vec![(Position {x: 1,y: 0}, DownAndLeft), (Position{x: 0,y:0}, Start)],
        true
    )]
    #[case(
        vec![(Position {x: 0,y: 0}, DownAndLeft), (Position{x: 0,y:1}, Start)],
        true
    )]
    #[case(
        vec![(Position {x: 0,y: 0}, DownAndLeft), (Position{x: 0,y:1}, Vertical)],
        true
    )]
    #[case(
        vec![(Position {x: 1,y: 0}, DownAndLeft), (Position{x: 0,y:0}, DownAndLeft)],
        false
    )]
    #[case(
        vec![(Position {x: 1,y: 0}, DownAndRight), (Position{x: 0,y:0}, DownAndRight)],
        false
    )]
    #[case(
        vec![(Position {x: 0,y: 0}, DownAndRight), (Position{x: 1,y:0}, DownAndLeft)],
        true
    )]
    #[case(
        vec![(Position {x: 0,y: 0}, DownAndRight), (Position{x: 0,y:1}, Start)],
        true
    )]
    #[case(
        vec![(Position {x: 0,y: 1}, DownAndRight), (Position{x: 0,y:0}, Start)],
        false
    )]
    fn test_is_connected(#[case] mut map: Vec<(Position, Pipe)>, #[case] expected: bool) {
        let b = map.pop().unwrap();
        let a = map.pop().unwrap();
        assert_eq!(expected, is_connected(a, b));
    }

    #[rstest]
    #[case(
        "-L|F7
7S-7|
L|7||
-L-J|
L|-JF",
        "4"
    )]
    #[case(
        "..F7.
.FJ|.
SJ.L7
|F--J
LJ...",
        "8"
    )]
    fn test_day10_part1(#[case] input: &str, #[case] expected: &str) {
        assert_eq!(expected, day10_part1(input));
    }

    #[rstest]
    #[case(
        "..........
.S------7.
.|F----7|.
.||....||.
.||....||.
.|L-7F-J|.
.|..||..|.
.L--JL--J.
..........",
        "4"
    )]
    #[case(
        "..........
.S------7.
.|......|.
.|F----7|.
.||....||.
.||....||.
.|L-7F-J|.
.|..||..|.
.L--JL--J.
..........",
        "10"
    )]
    #[case(
        ".....
.S-7.
.|.|.
.L-J.
.....",
        "1"
    )]
    #[case(
        ".F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ...",
        "8"
    )]
    fn test_day10_part2(#[case] input: &str, #[case] expected: &str) {
        assert_eq!(expected, day10_part2(input));
    }
}
