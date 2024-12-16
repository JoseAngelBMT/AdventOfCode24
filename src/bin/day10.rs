use std::collections::HashSet;
use AdventOfCode::board::{Board, Coord};

fn find_start_path(board: &Board<i32>) -> Vec<Coord> {
    let result = board
        .rows
        .iter()
        .enumerate()
        .flat_map(|(index, row)| {
            row.iter().enumerate().filter_map(move |(index2, value)| {
                if value == &0 {
                    Some(Coord::new(index2 as i32, index as i32))
                } else {
                    None
                }
            })
        })
        .collect();
    result
}

fn find_path_visited(board: &Board<i32>, actual: Coord, number: i32, visited: &mut HashSet<Coord>) -> i32 {
    if visited.contains(&actual){
        return 0;
    }
    visited.insert(actual);
    if board.get_value(actual).unwrap() == &9{
        return 1;
    }

    let mut result = 0;
    for coord in [Coord::new(0, 1), Coord::new(0, -1), Coord::new(1, 0), Coord::new(-1, 0)] {
        let next = actual + coord;
        if board.is_in_bound(next) && board.get_value(next).unwrap() == &number && !visited.contains(&next) {
            result += find_path_visited(board, next, number + 1, visited);
        }
    }
    result
}

fn part1(board: &Board<i32>) -> i32 {
    let mut sum = 0;
    for coord in find_start_path(board) {
        sum += find_path_visited(board, coord, 1, &mut HashSet::new());
    }
    sum
}

fn find_path(board: &Board<i32>, actual: Coord, number: i32) -> i32 {
    if board.get_value(actual).unwrap() == &9{
        return 1;
    }

    let mut result = 0;
    for coord in [Coord::new(0, 1), Coord::new(0, -1), Coord::new(1, 0), Coord::new(-1, 0)] {
        let next = actual + coord;
        if board.is_in_bound(next) && board.get_value(next).unwrap() == &number{
            result += find_path(board, next, number + 1);
        }
    }
    result
}

fn part2(board: &Board<i32>) -> i32 {
    let mut sum = 0;
    for coord in find_start_path(board) {
        sum += find_path(board, coord, 1);
    }
    sum
}

fn main() {
    let board = Board::read_int_board("data/day10.txt");
    println!("Part 1: {}", part1(&board));
    println!("Part 2: {}", part2(&board));
}
