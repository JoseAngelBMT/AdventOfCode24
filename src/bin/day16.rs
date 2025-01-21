use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};
use AdventOfCode::board::{Board, Coord};

#[derive(Clone, Debug, Eq, PartialEq)]
struct Node {
    coord: Coord,
    g_cost: usize,
    h_cost: usize,
    direction: i8,
}

impl Node {
    fn f_cost(&self) -> usize {
        self.g_cost + self.h_cost
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

fn get_coord_from_direction(coord: Coord, direction: i8) -> (Coord, i8) {
    match direction {
        0 => (coord.up(), 0),
        1 => (coord.right(), 1),
        2 => (coord.down(), 2),
        3 => (coord.left(), 3),
        _ => (coord, direction),
    }
}

fn get_neighbors(node: &Node) -> Vec<(Coord, i8)> {
    let coord = node.coord;
    let i = node.direction;
    vec![
        get_coord_from_direction(coord, i),
        get_coord_from_direction(coord, (i + 1).rem_euclid(4)),
        get_coord_from_direction(coord, (i - 1).rem_euclid(4)),
    ]
}
fn a_star(board: &Board<char>, start: Coord, end: Coord) -> Option<Vec<(Coord, usize)>> {
    let mut open_queue = BinaryHeap::new();
    let mut path: HashMap<Coord, (Coord, usize)> = HashMap::new();
    let mut g_scores: HashMap<Coord, usize> = HashMap::new();
    g_scores.insert(start, 0);

    open_queue.push(Node {
        coord: start,
        g_cost: 0,
        h_cost: heuristic(&start, &end),
        direction: 1,
    });

    while let Some(current_node) = open_queue.pop() {
        let current_coord = current_node.coord;

        if current_coord == end {
            let final_score = path.get(&current_coord).unwrap();
            let mut final_path = vec![(current_coord, final_score.1 + 1)];
            let mut current_coord = current_coord;
            while let Some(&previous) = path.get(&current_coord) {
                final_path.push(previous);
                current_coord = previous.0;
            }
            final_path.reverse();
            return Some(final_path);
        }

        let neighbors = get_neighbors(&current_node);
        for (neighbor, direction) in neighbors {
            let value = board.get_value(neighbor.clone());
            if matches!(value, None | Some(&'#')) {
                continue;
            }

            let g_score_ant = g_scores.get(&current_coord).unwrap_or(&usize::MAX);
            if g_score_ant < &g_scores.get(&neighbor).unwrap_or(&usize::MAX) {
                let mut score_direction = 0;
                if direction != current_node.direction {
                    score_direction += 1000;
                }
                let g_score_value: usize = *Some(g_score_ant).unwrap() + score_direction + 1;
                path.insert(neighbor, (current_coord, g_score_ant.clone()));
                g_scores.insert(neighbor.clone(), g_score_value);

                open_queue.push(Node {
                    coord: neighbor.clone(),
                    g_cost: g_score_value,
                    h_cost: heuristic(&neighbor, &current_coord),
                    direction,
                });
            }
        }
    }
    None
}

fn main() {
    let board = Board::read_char_board("data/day16.txt");
    let start: Coord = board.find_element('S').unwrap();
    let end: Coord = board.find_element('E').unwrap();

    let part1_path = a_star(&board, start, end).unwrap();
    let final_node = part1_path.last().unwrap();
    println!("Part1 {:}", final_node.1);
}

#[cfg(test)]
mod tests {
    use AdventOfCode::board::{Board, Coord};
    use crate::a_star;

    #[test]
    fn test_part1_maze1() {
        let board = Board::read_char_board("data/day16test.txt");
        let start: Coord = board.find_element('S').unwrap();
        let end: Coord = board.find_element('E').unwrap();
        let part1_path = a_star(&board, start, end).unwrap();
        let final_node = part1_path.last().unwrap();
        assert_eq!(final_node.1, 7036);
    }
}
