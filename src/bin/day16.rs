use itertools::Itertools;
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};
use AdventOfCode::board::{Board, Coord};

#[derive(Clone, Debug, Eq, PartialEq, Hash, Copy)]
struct Node {
    coord: Coord,
    cost: usize,
    direction: i8,
    h_cost: usize,
}

impl Node {
    fn f_cost(&self) -> usize {
        self.cost + self.h_cost
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
fn heuristic(start: &Coord, end: &Coord) -> usize {
    ((end.x - start.x).abs() + (end.y - start.y).abs()) as usize
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

fn get_neighbors(node: &Node, cost: &usize, end: &Coord) -> Vec<Node> {
    let coord = node.coord;
    let i = node.direction;
    let move_coord = get_coord_from_direction(coord, i);
    vec![
        Node {
            coord: move_coord,
            direction: i,
            cost: cost + 1,
            h_cost: heuristic(&move_coord, end),
        },
        Node {
            coord,
            direction: (i + 1).rem_euclid(4),
            cost: cost + 1000,
            h_cost: node.h_cost,
        },
        Node {
            coord,
            direction: (i - 1).rem_euclid(4),
            cost: cost + 1000,
            h_cost: node.h_cost,
        },
    ]
}

fn get_tiles(
    paths: HashMap<Node, HashSet<Node>>,
    best_cost: usize,
    start: &Coord,
    end: &Coord,
) -> usize {
    let mut tiles: HashSet<Coord> = HashSet::new();
    let mut visited = HashSet::new();
    let mut queue = VecDeque::new();

    let end_nodes: Vec<&Node> = paths
        .keys()
        .filter(|node| node.coord == *end && node.cost == best_cost)
        .collect();

    for node in end_nodes {
        let node_clone = node.clone();
        if visited.insert(node_clone) {
            tiles.insert(node_clone.coord);
            queue.push_back(node_clone);
        }
    }

    while let Some(current_node) = queue.pop_front() {
        if let Some(predecessors) = paths.get(&current_node) {
            for pred in predecessors {
                let pred_clone = pred.clone();
                if visited.insert(pred_clone) {
                    tiles.insert(pred_clone.coord);
                    queue.push_back(pred_clone);
                }
            }
        }
    }
    tiles.len()
}

fn a_star(board: &Board<char>, start: Coord, end: Coord) -> (usize, usize) {
    let mut part1_score = usize::MAX;
    let mut path: HashMap<Node, HashSet<Node>> = HashMap::new();
    let mut open_queue = BinaryHeap::new();
    let mut g_scores: HashMap<(Coord, i8), usize> = HashMap::new();

    g_scores.insert((start, 1), 0);
    open_queue.push(Node {
        coord: start,
        cost: 0,
        direction: 1,
        h_cost: heuristic(&start, &end),
    });

    while let Some(current_node) = open_queue.pop() {
        let current_coord = current_node.coord;

        if current_coord == end {
            if part1_score > current_node.cost {
                part1_score = current_node.cost;
            }
            continue;
        }

        let neighbors = get_neighbors(&current_node, &current_node.cost, &end);
        for neighbor_node in neighbors {
            let value = board.get_value(neighbor_node.coord);
            if !matches!(value, None | Some('#')) {
                let score = *g_scores
                    .get(&(neighbor_node.coord, neighbor_node.direction))
                    .unwrap_or(&usize::MAX);

                if neighbor_node.cost < score {
                    path.insert(neighbor_node.clone(), HashSet::from([current_node]));
                    g_scores.insert(
                        (neighbor_node.coord, neighbor_node.direction),
                        neighbor_node.cost,
                    );

                    open_queue.push(neighbor_node);
                } else if neighbor_node.cost == score {
                    path.entry(neighbor_node.clone())
                        .or_insert_with(|| HashSet::new())
                        .insert(current_node.clone());
                }
            }
        }
    }
    let tiles = get_tiles(path, part1_score, &start, &end);
    (part1_score, tiles)
}

fn main() {
    let board = Board::read_char_board("data/day16test.txt");
    let start: Coord = board.find_element('S').unwrap();
    let end: Coord = board.find_element('E').unwrap();

    let (part1, part2) = a_star(&board, start, end);

    println!("Part1 {}", part1);
    println!("Part2: {}", part2);

}

#[cfg(test)]
mod tests {
    use crate::a_star;
    use AdventOfCode::board::{Board, Coord};

    #[test]
    fn test_part1_maze1() {
        let board = Board::read_char_board("data/day16test.txt");
        let start: Coord = board.find_element('S').unwrap();
        let end: Coord = board.find_element('E').unwrap();
        let (part1, part2) = a_star(&board, start, end);
        assert_eq!(part1, 7036);
        assert_eq!(part2, 45);
    }
}
