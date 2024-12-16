use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

enum Operation {
    Split,
    Product,
}

fn calculate(x: &i64, operation: &Operation) -> Vec<i64> {
    match operation {
        Operation::Product => vec![x * 2024],
        Operation::Split => {
            let num_str = x.abs().to_string();
            let mid = num_str.len() / 2;
            let left = num_str[..mid].parse::<i64>().unwrap();
            let right = num_str[mid..].parse::<i64>().unwrap();
            vec![left, right]
        }
    }
}

fn read_file(path: &str) -> Vec<i64> {
    let file = File::open(path);
    let reader = BufReader::new(file.unwrap());
    reader
        .lines()
        .next()
        .expect("No lines found")
        .unwrap()
        .split_whitespace()
        .map(|val| val.parse::<i64>().expect("Not a number"))
        .collect()
}

fn even_digits(x: &i64) -> bool {
    let digits = x.abs().to_string().len();
    digits % 2 == 0
}

fn day11(row: Vec<i64>, blinks: i32) -> i64 {
    let mut stone_count: HashMap<i64, i64> = HashMap::new();
    for stone in row {
        stone_count
            .entry(stone)
            .and_modify(|e| *e += 1)
            .or_insert(1);
    }

    for _ in 0..blinks {
        let mut counts: HashMap<i64, i64> = HashMap::new();
        for (&stone, &count) in stone_count.iter() {
            if stone == 0 {
                *counts.entry(1).or_insert(0) += count;
            } else if even_digits(&stone) {
                let parts = calculate(&stone, &Operation::Split);
                *counts.entry(parts[0]).or_insert(0) += count;
                *counts.entry(parts[1]).or_insert(0) += count;
            } else {
                let parts = calculate(&stone, &Operation::Product);
                *counts.entry(parts[0]).or_insert(0) += count;
            }
        }
        stone_count = counts;
    }

    stone_count.values().sum()
}

fn main() {
    let row = read_file("data/day11.txt");
    println!("Part 1: {}", day11(row, 6));
}
