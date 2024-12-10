use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ops::{Add, Mul};
use std::collections::HashSet;
use std::hash::Hash;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
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

fn find_player(board: &Vec<Vec<char>>) -> Option<Coord> {
    let targets = ['<', '>', '^', 'v'];

    board
        .iter()
        .enumerate()
        .find_map(|(row_idx, row)| {
            row.iter()
                .enumerate()
                .find(|&(_, &cell)| targets.contains(&cell))
                .map(|(col_idx, _)| Coord {
                    y: row_idx as i32,
                    x: col_idx as i32,

                })
        })
}

fn get_value(matrix: &Vec<Vec<char>>, coord: Coord) -> Option<char> {
    let x: usize = coord.x.try_into().ok()?;
    let y: usize = coord.y.try_into().ok()?;

    matrix
        .get(y)
        .and_then(|row| row.get(x))
        .copied()
}

fn get_direction(board: &Vec<Vec<char>>, player: Coord) -> usize {
    match get_value(board, player) {
        Some('^') => 0,
        Some('>') => 1,
        Some('v') => 2,
        Some('<') => 3,
        _ => 0,
    }
}


fn loop_game(board: &Vec<Vec<char>>, directions: &[Coord]) -> Option<HashSet<(Coord, usize)>> {
    let mut visited_positions: HashSet<(Coord, usize)> = HashSet::new();
    let mut coord = find_player(board).unwrap();
    let mut i = get_direction(board, coord);
    loop{
        if !visited_positions.insert((coord, i)){
            return None
        }
        let next_coord = coord + directions[i];

        if let Some(value) = get_value(board, next_coord) {
            if value == '#' {
                i = (i + 1)%4;
            }else{
                coord = next_coord;
            }
        }else{
            break;
        }
    }
    Some(visited_positions)
}

fn part2(board: &Vec<Vec<char>>, visited: HashSet<Coord>, directions: &[Coord]) -> i32 {
    let mut count = 0;
    let coord = find_player(board).unwrap();
    for vis in visited{
        if coord != vis{
            let mut new_board = board.clone();
            new_board[vis.y as usize][vis.x as usize] = '#';
            if let None = loop_game(&new_board, &directions){
                count += 1;
            }
        }
    }
    count
}


fn main() {
    let directions = [Coord::new(0, -1), Coord::new(1, 0), Coord::new(0, 1), Coord::new(-1, 0)]; //^, >, v, <
    let board = read_file("data/day6.txt");
    let visited_positions = loop_game(&board, &directions).unwrap();
    let positions: HashSet<Coord> = visited_positions.into_iter().map(|(coord, _)| coord).collect();

    println!("Part 1: {}", positions.iter().count());
    println!("Part 2: {}", part2(&board, positions, &directions));


}