use glam::IVec2;
use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap, HashSet};

#[derive(Debug, Copy, Clone)]
struct Edge {
    to: IVec2,
    direction: IVec2, // as (0,1), (1,0), (-1, 0), or (0, -1)
    direction_count: u8,
    weight: u32,
}

impl PartialEq for Edge {
    fn eq(&self, other: &Self) -> bool {
        self.weight == other.weight
            && self.to == other.to
            && self.direction == other.direction
            && self.direction_count == other.direction_count
    }
}

impl Eq for Edge {}

impl PartialOrd for Edge {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.weight.partial_cmp(&other.weight)
    }
}

impl Ord for Edge {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other)
            .expect("partial cmp should always return a value")
    }
}

#[derive(Debug)]
struct Graph {
    nodes: HashMap<IVec2, u32>, // current_cost, this_node, prev_node
    seen_positions: HashSet<(IVec2, u8, IVec2)>, // position, direction count, direction. Intentionally doesn't include the cost.
    heap: BinaryHeap<Reverse<Edge>>,
}

fn build_graph(input: &str) -> Graph {
    let nodes = input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars().enumerate().map(move |(x, c)| {
                let position = IVec2 {
                    x: x as i32,
                    y: y as i32,
                };
                (
                    position,
                    c.to_digit(10).expect("only digit chars expected") as u32,
                )
            })
        })
        .collect::<HashMap<IVec2, u32>>();

    Graph {
        nodes,
        seen_positions: HashSet::new(),
        heap: BinaryHeap::new(),
    }
}

pub fn day17_part1(input: &str) -> String {
    let mut graph = build_graph(input);
    let max_x = (input.lines().nth(0).unwrap().chars().count() as i32) - 1;
    let max_y = (input.lines().count() as i32) - 1;
    let end = IVec2 { x: max_x, y: max_y };
    let beginning = IVec2 { x: 0, y: 0 };
    graph.heap.push(Reverse(Edge {
        to: beginning,
        direction: beginning,
        direction_count: 0,
        weight: 0,
    }));

    while graph.heap.len() > 0 {
        let next_edge = graph.heap.pop().expect("should have had an entry").0;

        if next_edge.to == end {
            return next_edge.weight.to_string();
        }

        if graph
            .seen_positions
            .get(&(next_edge.to, next_edge.direction_count, next_edge.direction))
            .is_some()
        {
            continue;
        }

        graph
            .seen_positions
            .insert((next_edge.to, next_edge.direction_count, next_edge.direction));

        if next_edge.direction != beginning && next_edge.direction_count < 3 {
            // push continuing to go straight on to the queue
            let next_position = next_edge.to + next_edge.direction;
            match graph.nodes.get(&next_position) {
                Some(next_node) => {
                    graph.heap.push(Reverse(Edge {
                        to: next_position,
                        direction: next_edge.direction,
                        direction_count: next_edge.direction_count + 1,
                        weight: next_edge.weight + next_node,
                    }));
                }
                None => (),
            }
        }

        for direction in vec![
            IVec2 { x: 0, y: 1 },
            IVec2 { x: 1, y: 0 },
            IVec2 { x: 0, y: -1 },
            IVec2 { x: -1, y: 0 },
        ] {
            // Can't go backwards and can't go forwards anymore
            if direction == next_edge.direction
                || direction
                    == (IVec2 {
                        x: -(next_edge.direction.x),
                        y: -(next_edge.direction.y),
                    })
            {
                continue;
            }
            let next_position = next_edge.to + direction;
            match graph.nodes.get(&next_position) {
                Some(next_node) => {
                    graph.heap.push(Reverse(Edge {
                        to: next_position,
                        direction,
                        direction_count: 1,
                        weight: next_edge.weight + next_node,
                    }));
                }
                None => (),
            }
        }
    }

    unreachable!("should have hit the end before draining the priority queue!")
}

pub fn day17_part2(input: &str) -> String {
    let mut graph = build_graph(input);
    let max_x = (input.lines().nth(0).unwrap().chars().count() as i32) - 1;
    let max_y = (input.lines().count() as i32) - 1;
    let end = IVec2 { x: max_x, y: max_y };
    let beginning = IVec2 { x: 0, y: 0 };
    graph.heap.push(Reverse(Edge {
        to: beginning,
        direction: beginning,
        direction_count: 0,
        weight: 0,
    }));

    while graph.heap.len() > 0 {
        let next_edge = graph.heap.pop().expect("should have had an entry").0;

        if next_edge.to == end && next_edge.direction_count >= 4 {
            return next_edge.weight.to_string();
        }

        if graph
            .seen_positions
            .get(&(next_edge.to, next_edge.direction_count, next_edge.direction))
            .is_some()
        {
            continue;
        }

        graph
            .seen_positions
            .insert((next_edge.to, next_edge.direction_count, next_edge.direction));

        if next_edge.direction != beginning && next_edge.direction_count < 10 {
            // push continuing to go straight on to the queue
            let next_position = next_edge.to + next_edge.direction;
            match graph.nodes.get(&next_position) {
                Some(next_node) => {
                    graph.heap.push(Reverse(Edge {
                        to: next_position,
                        direction: next_edge.direction,
                        direction_count: next_edge.direction_count + 1,
                        weight: next_edge.weight + next_node,
                    }));
                }
                None => (),
            }
        }

        // The ultra has to move in 4 places first.
        if next_edge.direction != beginning && next_edge.direction_count < 4 {
            continue;
        }

        for direction in vec![
            IVec2 { x: 0, y: 1 },
            IVec2 { x: 1, y: 0 },
            IVec2 { x: 0, y: -1 },
            IVec2 { x: -1, y: 0 },
        ] {
            // Can't go backwards and can't go forwards anymore
            if direction == next_edge.direction
                || direction
                    == (IVec2 {
                        x: -(next_edge.direction.x),
                        y: -(next_edge.direction.y),
                    })
            {
                continue;
            }
            let next_position = next_edge.to + direction;
            match graph.nodes.get(&next_position) {
                Some(next_node) => {
                    graph.heap.push(Reverse(Edge {
                        to: next_position,
                        direction,
                        direction_count: 1,
                        weight: next_edge.weight + next_node,
                    }));
                }
                None => (),
            }
        }
    }

    unreachable!("should have hit the end before draining the priority queue!")
}

#[cfg(test)]
mod test {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(
        "2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533",
        "102"
    )]
    fn test_day17_part1(#[case] input: &str, #[case] expected: &str) {
        assert_eq!(day17_part1(input), expected);
    }

    #[rstest]
    #[case(
        "2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533",
        "94"
    )]
    fn test_day17_part2(#[case] input: &str, #[case] expected: &str) {
        assert_eq!(day17_part2(input), expected);
    }
}
