use itertools::Itertools;
use std::cmp::PartialEq;
use std::fmt;
use std::path::Prefix::Disk;
use AdventOfCode::board::Board;
#[derive(Clone, Debug)]
enum DiskPosition {
    Id { id: i32 },
    Empty,
}

impl PartialEq for DiskPosition {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (DiskPosition::Id { id: id1 }, DiskPosition::Id { id: id2 }) => id1 == id2,
            (DiskPosition::Empty, DiskPosition::Empty) => true,
            _ => false,
        }
    }
}

impl fmt::Display for DiskPosition {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DiskPosition::Id { id } => write!(f, "{}", id),
            DiskPosition::Empty => write!(f, "."),
        }
    }
}

fn print(disk: &Vec<DiskPosition>) {
    for position in disk {
        print!("{}", position);
    }
    println!();
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

fn calculate_sum(disk: &Vec<DiskPosition>) -> i64 {
    disk.into_iter()
        .enumerate()
        .map(|(index, position)| {
            if let DiskPosition::Id { id } = position {
                (index as i64) * (*id as i64)
            } else {
                0
            }
        })
        .sum()
}

fn part1(disk: &Vec<DiskPosition>) -> i64 {
    let mut disk = disk.clone();
    move_blocks(&mut disk);
    calculate_sum(&disk)
}

fn group_by_id(disk: &Vec<DiskPosition>) -> Vec<Vec<DiskPosition>> {
    disk.iter().fold(vec![], |mut acc, curr| {
        match curr {
            DiskPosition::Id { .. } => {
                if let Some(last) = acc.last_mut() {
                    if last.last() == Some(curr) {
                        last.push(curr.clone());
                    } else {
                        acc.push(vec![curr.clone()]);
                    }
                } else {
                    acc.push(vec![curr.clone()]);
                }
            }
            _ => {
                acc.push(vec![curr.clone()]);
            }
        }
        acc
    })
}

fn find_right_chunk(disk: &Vec<DiskPosition>, last: usize) -> usize {
    if let DiskPosition::Id { id: starting_id } = disk[last] {
        for i in (0..=last).rev() {
            match &disk[i] {
                DiskPosition::Id { id } if *id == starting_id => continue,
                _ => return i + 1,
            }
        }
    }
    0
}

fn find_empty_chunk(disk: &Vec<DiskPosition>, i: usize, length: usize) -> Option<usize> {
    let mut count = 0;
    for (index, pos) in disk.iter().enumerate() {
        if i > index {
            match pos {
                DiskPosition::Empty => {
                    count += 1;
                    if count == length {
                        return Some(index - length + 1);
                    }
                }
                _ => {
                    count = 0;
                }
            }
        }
    }
    None
}

fn swap_chunks(disk: &mut Vec<DiskPosition>, left: usize, right: usize, length: usize) {
    for i in 0..length + 1 {
        disk.swap(left + i, right + i);
    }
}

fn move_chunks(disk: &mut Vec<DiskPosition>) {
    let mut right = disk.len() - 1;

    while 0 <= right {
        match disk[right] {
            DiskPosition::Id { id } => {
                if id == 0 {
                    break;
                }
                let start = find_right_chunk(&disk, right);
                let length = right - start;
                let empty_chunk = find_empty_chunk(&disk, right, length + 1);
                if let Some(empty_chunk) = empty_chunk {
                    swap_chunks(disk, start, empty_chunk, length);
                }
                right -= length + 1
            }
            _ => {
                if right == 0 {
                    break;
                }
                right -= 1;
            }
        }
    }
}

fn part2(disk: &Vec<DiskPosition>) -> i64 {
    let mut disk = disk.clone();
    move_chunks(&mut disk);
    calculate_sum(&disk)
}

fn main() {
    let disk = read_file("data/day9.txt");
    println!("Parte 1 {:?}", part1(&disk));
    println!("Parte 2 {:?}", part2(&disk));
}
