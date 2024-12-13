use AdventOfCode::board::{Board, Coord};

fn contains_word(board: &Board<char>, word: &str, coord: Coord, delta: Coord) -> bool {
    if word.is_empty() {
        return true;
    }

    if let Some(val) = board.get_value(coord) {
        let (head, tail) = word.split_at(1);
        let head = head.chars().next().unwrap();
        return head == val.clone() && contains_word(board, tail, coord+delta, delta);
    }
    false
}

fn part1(board: &Board<char>, word: &str) -> i32 {
    let mut count = 0;
    let directions = [Coord::new(1, 0), Coord::new(0, 1), Coord::new(-1, 0), Coord::new(0, -1),
                                Coord::new(-1, 1), Coord::new(1, -1), Coord::new(-1, -1), Coord::new(1, 1)];
    for i in 0..board.rows.len() {
        for j in 0..board.rows[0].len() {
            let coord = Coord::new(i as i32, j as i32);
            for dir in directions.iter() {
                if contains_word(board, word, coord, *dir){
                    count += 1;
                }
            }
        }
    }
    count
}

fn check_diagonal(board: &Board<char>, coord: Coord, delta: Coord) -> bool {
    let inverse_coord = Coord::new(-1, -1);
    let reverse: Coord = delta * inverse_coord;
    if let (Some(val), Some(val_inv)) = (board.get_value(coord + delta), board.get_value(coord + reverse)) {
        if (val.clone() == 'M' && val_inv.clone() == 'S') || (val.clone() == 'S' && val_inv.clone() == 'M') {
            return true;
        }
    }
    false
}

fn part2(board: &Board<char>) -> i32 {

    let mut count = 0;
    let directions = [Coord::new(1, -1), Coord::new(1, 1)];
    for i in 0..board.rows.len() {
        for j in 0..board.rows[0].len() {
            let coord = Coord::new(i as i32, j as i32);
            if let Some(val) = board.get_value(coord) {
                if val.clone() == 'A' && check_diagonal(board, coord, directions[0]) && check_diagonal(board, coord, directions[1]) {
                    count += 1;
                }
            }
        }
    }
    count
}

fn main() {
    let matrix = Board::read_char_board("data/day4.txt");
    println!("{}", part1(&matrix, "XMAS"));
    println!("{}", part2(&matrix));
}