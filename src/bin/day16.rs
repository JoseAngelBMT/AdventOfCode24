use std::cmp::Ordering;
use std::collections::{HashMap, HashSet, VecDeque};
use AdventOfCode::board::{Board, Coord};
use AdventOfCode::path_finding::*;

#[derive(Clone, Debug, Eq, PartialEq, Hash, Copy)]
struct State {
    coord: Coord,
    direction: i8,
}

impl State {
    fn new(coord: Coord, direction: i8) -> Self {
        Self { coord, direction }
    }
}

impl StateTrait for State {
    fn get_coord(&self) -> Coord {
        self.coord.clone()
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Hash, Copy)]
struct Node {
    state: State,
    cost: usize,
    h_cost: usize,
}

impl Node {
    fn f_cost(&self) -> usize {
        self.cost + self.h_cost
    }
}

impl NodeTrait for Node {
    type State = State;
    fn get_state(&self) -> State {
        self.state.clone()
    }

    fn get_cost(&self) -> usize {
        self.cost
    }

    fn get_h_cost(&self) -> usize {
        self.h_cost
    }

    fn new(state: Self::State, cost: usize, h_cost: usize) -> Self {
        Self {
            state,
            cost,
            h_cost,
        }
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        other.f_cost().cmp(&self.f_cost())
    }
}

impl PartialOrd<Self> for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

// Manhattan distance
fn heuristic(start: &Coord, end: &State) -> usize {
    ((end.coord.x - start.x).abs() + (end.coord.y - start.y).abs()) as usize
}

fn get_coord_from_direction(coord: Coord, direction: i8) -> Coord {
    match direction {
        0 => coord.up(),
        1 => coord.right(),
        2 => coord.down(),
        3 => coord.left(),
        _ => panic!("Invalid direction"),
    }
}

fn get_neighbors(node: &Node, end: &State) -> Vec<Node> {
    let coord = node.state.coord;
    let i = node.state.direction;
    let move_coord = get_coord_from_direction(coord, i);
    vec![
        Node {
            state: State::new(move_coord, i),
            cost: node.cost + 1,
            h_cost: heuristic(&move_coord, end),
        },
        Node {
            state: State::new(coord, (i + 1).rem_euclid(4)),
            cost: node.cost + 1000,
            h_cost: node.h_cost,
        },
        Node {
            state: State::new(coord, (i - 1).rem_euclid(4)),
            cost: node.cost + 1000,
            h_cost: node.h_cost,
        },
    ]
}

fn get_tiles(paths: HashMap<Node, HashSet<Node>>, best_cost: usize, end: &Coord) -> usize {
    let mut tiles: HashSet<Coord> = HashSet::new();
    let mut visited = HashSet::new();
    let mut queue = VecDeque::new();

    let end_nodes: Vec<&Node> = paths
        .keys()
        .filter(|node| node.state.coord == *end && node.cost == best_cost)
        .collect();

    for node in end_nodes {
        let node_clone = node.clone();
        if visited.insert(node_clone) {
            tiles.insert(node_clone.state.coord);
            queue.push_back(node_clone);
        }
    }

    while let Some(current_node) = queue.pop_front() {
        if let Some(predecessors) = paths.get(&current_node) {
            for pred in predecessors {
                let pred_clone = pred.clone();
                if visited.insert(pred_clone) {
                    tiles.insert(pred_clone.state.coord);
                    queue.push_back(pred_clone);
                }
            }
        }
    }
    tiles.len()
}

fn main() {
    let board = Board::read_char_board("data/day16.txt");
    let start: Coord = board.find_element('S').unwrap();
    let end: Coord = board.find_element('E').unwrap();

    let (part1, paths) = a_star(
        &board,
        State::new(start, 1),
        State::new(end, 0),
        get_neighbors,
        false
    );
    let part2 = get_tiles(paths, part1, &end);
    println!("Part1 {}", part1);
    println!("Part2: {}", part2);
}

#[cfg(test)]
mod tests {
    use crate::{a_star, get_neighbors, get_tiles, State};
    use AdventOfCode::board::{Board, Coord};

    #[test]
    fn test_part1_maze1() {
        let board = Board::read_char_board("data/day16test.txt");
        let start: Coord = board.find_element('S').unwrap();
        let end: Coord = board.find_element('E').unwrap();
        let (part1, paths) = a_star(
            &board,
            State::new(start, 1),
            State::new(end, 0),
            get_neighbors,
            false
        );
        let part2 = get_tiles(paths, part1, &end);
        assert_eq!(part1, 7036);
        assert_eq!(part2, 45);
    }
}
