use std::collections::HashSet;
use AdventOfCode::board::{Board, Coord};

fn surrounding(board: &Board<char>, coord: &Coord, delta: &Coord) -> bool {
    let value = board.get_value(*coord).unwrap();
    let count = [
        Coord::new(coord.x + delta.x, coord.y),
        Coord::new(coord.x, coord.y + delta.y),
    ]
    .iter()
    .map(|&adj_coord| board.get_value(adj_coord))
    .map(|opt_value| match opt_value {
        Some(v) if v != value => 1,
        None => 1,
        _ => 0,
    })
    .sum::<i32>();

    if let Some(v) = board.get_value(*coord + *delta) {
        if value == v {
            return count == 2;
        }
    }
    count == 0 || count == 2
}

fn count_corners(board: &Board<char>, coord: &Coord) -> i32 {
    vec![
        Coord::new(-1, -1),
        Coord::new(1, -1),
        Coord::new(1, 1),
        Coord::new(-1, 1),
    ]
    .iter()
    .filter(|x| surrounding(board, coord, x))
    .count() as i32
}

fn explore(
    board: &Board<char>,
    coord: &Coord,
    value: &char,
    visited: &mut HashSet<Coord>,
    perimeter: &mut i32,
    corners: &mut i32,
) -> i32 {
    if visited.contains(coord) {
        return 0;
    }
    visited.insert(*coord);
    *corners += count_corners(board, coord);

    let mut area = 1;
    let deltas = vec![
        Coord::new(-1, 0),
        Coord::new(1, 0),
        Coord::new(0, -1),
        Coord::new(0, 1),
    ];
    for delta in deltas {
        let neighbor = *coord + delta;
        match board.get_value(neighbor) {
            Some(v) if v == value => {
                area += explore(board, &neighbor, value, visited, perimeter, corners);
            }
            _ => *perimeter += 1,
        }
    }
    area
}

fn find_area_and_perimeter(
    board: &Board<char>,
    coord: &Coord,
    visited: &mut HashSet<Coord>,
) -> (i32, i32, i32) {
    if let Some(value) = board.get_value(*coord) {
        let mut perimeter = 0;
        let mut corners = 0;
        let area = explore(board, coord, &value, visited, &mut perimeter, &mut corners);
        (area, perimeter, corners)
    } else {
        (0, 0, 0)
    }
}
fn day12(board: &Board<char>) -> (i32, i32) {
    let (mut part1, mut part2) = (0, 0);
    let mut visited = HashSet::new();
    for (y, row) in board.rows.iter().enumerate() {
        for (x, _) in row.iter().enumerate() {
            let coord = Coord::new(x as i32, y as i32);
            if visited.contains(&coord) {
                continue;
            }
            let (area, perimeter, corners) = find_area_and_perimeter(board, &coord, &mut visited);
            part1 += area * perimeter;
            part2 += area * corners;
        }
    }
    (part1, part2)
}

fn main() {
    let board = Board::read_char_board("data/day12.txt");
    let (part1, part2) = day12(&board);
    println!("Parte 1: {}", part1);
    println!("Parte 2: {}", part2);
}
