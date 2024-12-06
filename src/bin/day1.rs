use std::fs::File;
use std::io::{BufRead, BufReader};

fn read_file(path: &str) -> Vec<Vec<i32>> {
    let mut left: Vec<i32> = Vec::new();
    let mut right: Vec<i32> = Vec::new();

    let file = File::open(path);
    let reader = BufReader::new(file.unwrap());
    for line in reader.lines() {
        let numbers: Vec<i32> = line.unwrap()
            .split_whitespace()
            .map(|s| s.parse::<i32>().unwrap())
            .collect();
        left.push(numbers[0]);
        right.push(numbers[1]);
    }

    right.sort();
    left.sort();

    let matrix: Vec<Vec<i32>> = vec![left, right];
    matrix
}

fn part1(mut left: Vec<i32>, mut right: Vec<i32>) -> i32 {
    let mut sum: i32 = 0;
    while right.is_empty() == false {
        let right_val = right[0];
        let left_val = left[0];
        sum += (right_val - left_val).abs();

        right.remove(0);
        left.remove(0);
    }
    sum
}

fn part2(mut left: Vec<i32>, mut right: Vec<i32>) -> i32 {
    let mut sum: i32 = 0;
    for i in 0..left.len() {
        let left_val = left[i];
        let right_count = right.iter()
            .filter(|&&x| x==left_val)
            .count() as i32;
        sum += left_val * right_count;
    }
    sum
}

fn main() {
    let matrix: Vec<Vec<i32>> = read_file("data/day1.txt");
    let mut left: Vec<i32> = matrix[0].clone();
    let mut right: Vec<i32> = matrix[1].clone();

    let part1_sol = part1(left.clone(), right.clone());
    println!("Part 1 solution: {}", part1_sol);

    let part2_sol = part2(left.clone(), right.clone());
    println!("Part 2 solution: {}", part2_sol);
}
