use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ops::{Add, Mul};

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct Coord {
    x: i32,
    y: i32,
}

impl Coord {
    fn new(x: i32, y: i32) -> Coord {
        Coord { x, y }
    }
}

impl Add for Coord {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl Mul for Coord {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self::new(self.x * rhs.x, self.y * rhs.y)
    }
}

fn read_file(path: &str) -> Vec<Vec<char>> {
    let file = File::open(path);
    let reader = BufReader::new(file.unwrap());

    reader
        .lines()
        .map(|line| {
            line
                .unwrap()
                .chars()
                .collect()
        })
        .collect()
}

fn get_value(matrix: &Vec<Vec<char>>, coord: Coord) -> Option<char> {
    let x: usize = coord.x.try_into().ok()?;
    let y: usize = coord.y.try_into().ok()?;

    matrix
        .get(y)
        .and_then(|row| row.get(x))
        .copied()
}

fn contains_word(matrix: &Vec<Vec<char>>, word: &str, coord: Coord, delta: Coord) -> bool {
    if word.is_empty() {
        return true;
    }

    if let Some(val) = get_value(matrix, coord) {
        let (head, tail) = word.split_at(1);
        let head = head.chars().next().unwrap();
        return head == val && contains_word(matrix, tail, coord+delta, delta);
    }
    false
}


fn part1(matrix: &Vec<Vec<char>>, word: &str) -> i32 {
    let mut count = 0;
    let directions = [Coord::new(1, 0), Coord::new(0, 1), Coord::new(-1, 0), Coord::new(0, -1),
                                Coord::new(-1, 1), Coord::new(1, -1), Coord::new(-1, -1), Coord::new(1, 1)];
    for i in 0..matrix.len() {
        for j in 0..matrix[0].len() {
            let coord = Coord::new(i as i32, j as i32);
            for dir in directions.iter() {
                if contains_word(matrix, word, coord, *dir){
                    count += 1;
                }
            }
        }
    }
    count
}

fn check_diagonal(matrix: &Vec<Vec<char>>, coord: Coord, delta: Coord) -> bool {
    let inverse_coord = Coord::new(-1, -1);
    let reverse: Coord = delta * inverse_coord;
    if let (Some(val), Some(val_inv)) = (get_value(matrix, coord + delta), get_value(matrix, coord + reverse)) {
        if (val == 'M' && val_inv == 'S') || (val == 'S' && val_inv == 'M') {
            return true;
        }
    }
    false
}

fn part2(matrix: &Vec<Vec<char>>) -> i32 {

    let mut count = 0;
    let directions = [Coord::new(1, -1), Coord::new(1, 1)];
    for i in 0..matrix.len() {
        for j in 0..matrix[0].len() {
            let coord = Coord::new(i as i32, j as i32);
            if let Some(val) = get_value(matrix, coord) {
                if val == 'A' && check_diagonal(matrix, coord, directions[0]) && check_diagonal(matrix, coord, directions[1]) {
                    count += 1;
                }
            }
        }
    }
    count
}

fn main() {
    let matrix = read_file("data/day4.txt");
    println!("{}", part1(&matrix, "XMAS"));
    println!("{}", part2(&matrix));
}