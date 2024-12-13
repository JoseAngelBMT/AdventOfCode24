use std::collections::HashSet;
use AdventOfCode::board::{Board, Coord};

fn find_player(board: &Board<char>) -> Option<Coord> {
    let targets = ['<', '>', '^', 'v'];

    board
        .rows
        .iter()
        .enumerate()
        .find_map(|(row_idx, row)| {
            row.iter()
                .enumerate()
                .find(|&(_, &cell)| targets.contains(&cell))
                .map(|(col_idx, _)| Coord::new(
                    col_idx as i32,
                    row_idx as i32,
                ))
        })
}


fn get_direction(board: &Board<char>, player: Coord) -> usize {
    match board.get_value(player) {
        Some('^') => 0,
        Some('>') => 1,
        Some('v') => 2,
        Some('<') => 3,
        _ => 0,
    }
}


fn loop_game(board: &Board<char>, directions: &[Coord]) -> Option<HashSet<(Coord, usize)>> {
    let mut visited_positions: HashSet<(Coord, usize)> = HashSet::new();
    let mut coord = find_player(board).unwrap();
    let mut i = get_direction(board, coord);
    loop{
        if !visited_positions.insert((coord, i)){
            return None
        }
        let next_coord = coord + directions[i];

        if let Some(value) = board.get_value(next_coord) {
            if value.clone() == '#' {
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

fn part2(board: &Board<char>, visited: HashSet<Coord>, directions: &[Coord]) -> i32 {
    let mut count = 0;
    let coord = find_player(board).unwrap();
    for vis in visited{
        if coord != vis{
            let mut new_board = board.clone();
            new_board.set_value(vis, '#');
            if let None = loop_game(&new_board, &directions){
                count += 1;
            }
        }
    }
    count
}

fn main() {
    let directions = [Coord::new(0, -1), Coord::new(1, 0), Coord::new(0, 1), Coord::new(-1, 0)]; //^, >, v, <
    let board = Board::read_char_board("data/day6.txt");
    let visited_positions = loop_game(&board, &directions).unwrap();
    let positions: HashSet<Coord> = visited_positions.into_iter().map(|(coord, _)| coord).collect();

    println!("Part 1: {}", positions.iter().count());
    println!("Part 2: {}", part2(&board, positions, &directions));


}