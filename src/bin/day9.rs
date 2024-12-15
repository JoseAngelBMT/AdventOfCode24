use std::fmt;
use AdventOfCode::board::Board;
#[derive(Clone, Debug)]
enum DiskPosition {
    Id { id: i32 },
    Empty,
}

impl fmt::Display for DiskPosition {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DiskPosition::Id { id } => write!(f, "{}", id),
            DiskPosition::Empty => write!(f, "."),
        }
    }
}

fn generate_positions(index: usize, value: i32) -> Vec<DiskPosition> {
    let position = if index % 2 == 0 {
        DiskPosition::Id {
            id: (index as i32) / 2,
        }
    } else {
        DiskPosition::Empty
    };
    vec![position; value as usize]
}

fn generate_disk_map(disk: &Vec<i32>) -> Vec<DiskPosition> {
    disk.into_iter()
        .enumerate()
        .flat_map(|(index, value)| generate_positions(index, *value))
        .collect()
}

fn read_file(path: &str) -> Vec<DiskPosition> {
    let numbers = Board::read_int_board(path).rows.first().unwrap().clone();
    generate_disk_map(&numbers)
}


fn move_blocks(disk: &mut Vec<DiskPosition>) {
    let mut left = 0;
    let mut right = disk.len() - 1;

    while left < right {
        while left < disk.len() && matches!(disk[left], DiskPosition::Id { .. }) {
            left += 1;
        }
        while right > 0 && matches!(disk[right], DiskPosition::Empty) {
            right -= 1;
        }
        if left < right {
            disk.swap(left, right);
            left += 1;
            right -= 1;
        }
    }
}

fn part1(disk: &Vec<DiskPosition>) -> i64 {
    let mut disk = disk.clone();
    move_blocks(&mut disk);
    disk.into_iter()
        .enumerate()
        .map(|(index, position)| {
            if let DiskPosition::Id { id } = position {
                (index as i64) * (id as i64)
            } else {
                0
            }
        })
        .sum()
}

fn main() {
    let disk = read_file("data/day9.txt");
    println!("Parte 1 {:?}", part1(&disk));
}
