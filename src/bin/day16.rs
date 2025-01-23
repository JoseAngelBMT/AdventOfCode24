use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet};
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
fn a_star(board: &Board<char>, start: Coord, end: Coord) -> (usize, Vec<Vec<(Coord, usize)>>) {
    let mut part1_score = usize::MAX;
    let mut paths: Vec<Vec<(Coord, usize)>> = Vec::new();
    let mut path: HashMap<Node, Node> = HashMap::new();
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
            let mut final_path = vec![(current_coord, current_node.cost)];
            let mut current = current_node.clone();
            while let Some(previous) = path.get(&current) {
                final_path.push((previous.coord, previous.cost));
                current = previous.clone();
            }
            path.clear();
            final_path.reverse();
            paths.push(final_path);
            continue;
        }

        let neighbors = get_neighbors(&current_node, &current_node.cost, &end);
        for neighbor_node in neighbors {
            let value = board.get_value(neighbor_node.coord);
            if !matches!(value, None | Some('#'))
                && current_node.cost
                < *g_scores
                .get(&(neighbor_node.coord, neighbor_node.direction))
                .unwrap_or(&usize::MAX)
            {
                path.insert(neighbor_node.clone(), current_node.clone());
                g_scores.insert(
                    (neighbor_node.coord, neighbor_node.direction),
                    neighbor_node.cost,
                );

                open_queue.push(neighbor_node);
            }
        }
    }
    paths.retain(|p| p.last().map(|(_, c)| *c == part1_score).unwrap_or(false));
    (part1_score, paths)
}

fn main() {
    let board = Board::read_char_board("data/day16test2.txt");
    let start: Coord = board.find_element('S').unwrap();
    let end: Coord = board.find_element('E').unwrap();

    let (part1, paths) = a_star(&board, start, end);

    println!("Paths: {}", paths.len());

    let tiles: HashSet<Coord> = paths
        .into_iter()
        .flat_map(|inner_vec| inner_vec.into_iter().map(|(coord, _)| coord))
        .collect();

    println!("Part1 {}", part1);
    println!("Part2: {}", tiles.len());

    let mut board_mut = board.clone();
    for tile in tiles {
        board_mut.set_value(tile, 'O');
    }
    board_mut.print_board();
}

#[cfg(test)]
mod tests {
    use crate::{a_star};
    use AdventOfCode::board::{Board, Coord};

    #[test]
    fn test_part1_maze1() {
        let board = Board::read_char_board("data/day16test.txt");
        let start: Coord = board.find_element('S').unwrap();
        let end: Coord = board.find_element('E').unwrap();
        let (part1, _) = a_star(&board, start, end);
        assert_eq!(part1, 7036);
    }
}
