use itertools::Itertools;
use std::collections::{HashMap, HashSet};
use AdventOfCode::board::{Board, Coord};
type MapCoords = HashMap<char, Vec<Coord>>;

struct Antinode {
    coord: Coord,
    delta: Coord,
}

impl Antinode {
    fn new(coord: Coord, delta: Coord) -> Self {
        Self { coord, delta }
    }
}

impl Iterator for Antinode {
    type Item = Coord;

    fn next(&mut self) -> Option<Self::Item> {
        let coord = self.coord;
        self.coord = self.coord + self.delta;
        Some(coord)
    }
}

fn create_map_coords(board: &Board<char>) -> MapCoords {
    let mut map: MapCoords = HashMap::new();
    for (row_idx, row) in board.rows.iter().enumerate() {
        for (col_idx, cell) in row.iter().enumerate() {
            if cell != &'.' {
                if let Some(val) = map.get_mut(cell) {
                    val.push(Coord::new(col_idx as i32, row_idx as i32));
                } else {
                    map.insert(
                        cell.clone(),
                        vec![Coord::new(col_idx as i32, row_idx as i32)],
                    );
                }
            }
        }
    }
    map
}

fn opposite_coord(coord1: &Coord, coord2: &Coord) -> Coord {
    Coord::new(
        coord1.x - (coord2.x - coord1.x),
        coord1.y - (coord2.y - coord1.y),
    )
}

fn part2(board: &Board<char>, map: &MapCoords) -> i32 {
    map.values()
        .flat_map(|v| v.iter().permutations(2))
        .flat_map(|pair| {
            let delta = *pair[1] - *pair[0];
            Antinode::new(*pair[1], delta).take_while(|c| board.is_in_bound(*c))
        })
        .unique()
        .count() as i32
}

fn part1(board: &Board<char>, map: &MapCoords) -> i32 {
    map.values()
        .flat_map(|v| v.iter().permutations(2))
        .map(|pair| opposite_coord(&pair[0], &pair[1]))
        .filter(|opp| board.is_in_bound(*opp))
        .unique()
        .count() as i32
}

fn main() {
    let board = Board::read_char_board("data/day8.txt");
    let map = create_map_coords(&board);
    // println!("{:?}", map);
    println!("Part 1: {}", part1(&board, &map));
    println!("Part 2: {}", part2(&board, &map));
}
