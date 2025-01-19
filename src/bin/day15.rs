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

fn modify_board(board: &mut Board<char>) {
    board.rows.iter_mut().for_each(|row| {
        let mut i = 0;
        while i < row.len() {
            match row[i] {
                '.' => {
                    row.insert(i + 1, '.');
                    i += 2;
                }
                'O' => {
                    row[i] = '[';
                    row.insert(i + 1, ']');
                    i += 2;
                }
                '#' => {
                    row.insert(i + 1, '#');
                    i += 2;
                }
                '@' => {
                    row.insert(i + 1, '.');
                    i += 2;
                }
                _ => {
                    i += 1;
                }
            }
        }
    });
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
    board.set_value(*coord1, *board.get_value(*coord2).unwrap());
    board.set_value(*coord2, coord1_value);
}

fn push_box(
    board: &mut Board<char>,
    coord: &Coord,
    movement: &Coord,
    first_coordinate: &Coord,
) -> bool {
    let next_coord = *coord + *movement;
    match board.get_value(next_coord) {
        Some('.') => {
            swap_values(board, coord, &next_coord);
            true
        }
        Some('O') => {
            let result = push_box(board, &next_coord, movement, first_coordinate);
            swap_values(board, coord, &next_coord);
            result
        }
        _ => false,
    }
}

fn push_large_box(
    board: &mut Board<char>,
    part1: &Coord,
    part2: &Coord,
    movement: &Coord,
    first_coordinate: &Coord,
) -> bool {
    let part2_next = *part2 + *movement;
    if movement.y == 0 {
        let part2_value = board.get_value(part2_next);
        match part2_value {
            Some('.') => {
                swap_values(board, part2, &part2_next);
                swap_values(board, part1, part2);
                true
            }
            Some('[') | Some(']') => {
                let next_part = match part2_value {
                    Some('[') => part2_next.right(),
                    Some(']') => part2_next.left(),
                    _ => part2_next
                };
                let result = push_large_box(board, &part2_next, &next_part, movement, first_coordinate);
                swap_values(board, part2, &part2_next);
                swap_values(board, part1, part2);
                result
            }
            _ => false
        }
    } else {
        let part1_next = *part1 + *movement;
        let part1_value = board.get_value(part1_next);
        let part2_value = board.get_value(part2_next);

        match (part1_value, part2_value) {
            (Some('.'), Some('.')) => {
                swap_values(board, part2, &part2_next);
                swap_values(board, part1, &part1_next);
                true
            }

            (Some('[') | Some(']') | Some('.'), Some('[') | Some(']') | Some('.')) => {
                let (next_part, next_part_2) = (
                    match part1_value {
                        Some('[') => part1_next.right(),
                        Some(']') => part1_next.left(),
                        _ => part1_next,
                    },
                    match part2_value {
                        Some('[') => part2_next.right(),
                        Some(']') => part2_next.left(),
                        _ => part2_next,
                    },
                );

                // If box ^ or v is in the same space of actual box
                // []
                // []
                if part1_value == board.get_value(*part1) {
                    let result = push_large_box(board, &part1_next, &next_part, movement, first_coordinate);
                    swap_values(board, part2, &part2_next);
                    swap_values(board, part1, &part1_next);
                    result
                } else {
                    let mut result = true;
                    if next_part != part1_next {
                        result = push_large_box(board, &part1_next, &next_part, movement, first_coordinate);
                    }
                    if next_part_2 != part2_next {
                        result = result & push_large_box(board, &part2_next, &next_part_2, movement, first_coordinate);
                    }
                    swap_values(board, part2, &part2_next);
                    swap_values(board, part1, &part1_next);
                    result
                }
            }
            _ => false,
        }
    }
}

fn next_state(board: &mut Board<char>, coord: &mut Coord, move_dir: &char) {
    let movement = char_to_coord(move_dir.clone()).unwrap();
    let next_coord = movement + coord.clone();
    let board_copy = board.clone();
    match board.get_value(next_coord) {
        Some('.') => {
            *coord = next_coord
        }
        Some('O') => {
            if push_box(board, &next_coord, &movement, &next_coord){
                *coord = next_coord
            }else{
                *board = board_copy;
            }
        }
        Some('[') => {
            if push_large_box(board, &next_coord, &next_coord.right(), &movement, &next_coord) {
                *coord = next_coord;
            }else{
                *board = board_copy;
            }
        }
        Some(']') => {
            if push_large_box(board, &next_coord, &next_coord.left(), &movement, &next_coord) {
                *coord = next_coord;
            }else{
                *board = board_copy;
            }
        }
        _ => {}
    }
}

fn count_coords(board: &Board<char>, element: char) -> i32 {
    board
        .rows
        .iter()
        .enumerate()
        .flat_map(|(y, row)| {
            row.iter().enumerate().filter_map(move |(x, &cell)| {
                if cell == element {
                    Some((y as i32) * 100 + (x as i32))
                } else {
                    None
                }
            })
        })
        .sum()
}

fn day15(board: &Board<char>, chars: &Vec<char>, coord: &Coord, element: char) -> i32 {
    let mut board = board.clone();
    board.set_value(*coord, '.');
    let mut coord = coord.clone();
    chars.iter().for_each(|c| {
        next_state(&mut board, &mut coord, c);
    });

    count_coords(&board, element)
}

fn main() {
    let (mut board, chars) = read_file("data/day15.txt");
    let player = board.find_element('@').unwrap();
    println!("Part 1: {:?}", day15(&board, &chars, &player, 'O'));

    modify_board(&mut board);
    let player = board.find_element('@').unwrap();
    println!("Part 2: {:?}", day15(&board, &chars, &player, '['));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day15_part2() {
        let (mut board, chars) = read_file("data/day15test.txt");
        modify_board(&mut board);
        let player = board.find_element('@').unwrap();
        assert_eq!(day15(&board, &chars, &player, '['), 9021);
    }

    #[test]
    fn test_day15_part2_solution() {
        let (mut board, chars) = read_file("data/day15.txt");
        modify_board(&mut board);
        let player = board.find_element('@').unwrap();
        assert_eq!(day15(&board, &chars, &player, '['), 1509780);
    }
}
