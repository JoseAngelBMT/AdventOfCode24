use AdventOfCode::board::{Board, Coord};
fn read_file(path: &str) -> (Board<char>, Vec<char>) {
    let content = std::fs::read_to_string(path).expect("Failed to read file");
    let mut parts = content.split("\n\n");

    let board_str = parts.next().expect("Missing Board part");
    let board = Board::from_string(board_str);

    let second_part = parts.next().expect("Missing second part");
    let chars: Vec<char> = second_part.chars().filter(|c| !c.is_whitespace()).collect();

    (board, chars)
}
fn char_to_coord(d: char) -> Option<Coord> {
    match d {
        '^' => Some(Coord { x: 0, y: -1 }),
        'v' => Some(Coord { x: 0, y: 1 }),
        '<' => Some(Coord { x: -1, y: 0 }),
        '>' => Some(Coord { x: 1, y: 0 }),
        _ => None,
    }
}

fn swap_values(board: &mut Board<char>, coord1: &Coord, coord2: &Coord) {
    let coord1_value = *board.get_value(*coord1).unwrap();
    let coord2_value = *board.get_value(*coord2).unwrap();
    board.set_value(*coord1, coord2_value);
    board.set_value(*coord2, coord1_value);
}

fn push_box(
    board: &mut Board<char>,
    coord: &Coord,
    movement: &Coord,
    first_coordinate: &Coord,
) -> Coord {
    let next_coord = *coord + *movement;
    match board.get_value(next_coord) {
        Some('.') => {
            swap_values(board, coord, &next_coord);
            *first_coordinate
        }
        Some('O') => {
            let result = push_box(board, &next_coord, movement, first_coordinate);
            swap_values(board, coord, &next_coord);
            result
        }
        _ => *first_coordinate - *movement,
    }
}

fn next_state(board: &mut Board<char>, coord: &mut Coord, move_dir: &char) {
    let movement = char_to_coord(*move_dir).unwrap();
    let next_coord = movement + *coord;
    if let Some(cell) = board.get_value(next_coord) {
        match cell {
            '#' => {
                *coord = *coord;
            }
            'O' => {
                *coord = push_box(board, &next_coord, &movement, &next_coord);
            }
            _ => {
                *coord = next_coord;
            }
        }
    } else {
        *coord = *coord;
    }
}

fn count_coords(board: &Board<char>) -> i32 {
    board
        .rows
        .iter()
        .enumerate()
        .flat_map(|(y, row)| {
            row.iter()
                .enumerate()
                .filter_map(move |(x, &cell)| if cell == 'O' {
                    Some((y as i32) * 100 + (x as i32))
                } else {
                    None
                })
        })
        .sum()
}

fn part1(board: &Board<char>, chars: &Vec<char>, coord: &Coord) -> i32 {
    let mut board = board.clone();
    let mut coord = coord.clone();
    chars.iter().for_each(|c| {
        next_state(&mut board, &mut coord, c);
    });
    println!();
    board.print_board();
    println!();
    count_coords(&board)
}

fn main() {
    let (board, chars) = read_file("data/day15test.txt");
    let lantern = board.find_element('@').unwrap();
    println!("Part 1: {:?}", part1(&board, &chars, &lantern));
}
