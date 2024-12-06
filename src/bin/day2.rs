use std::fs::File;
use std::io::{BufRead, BufReader};

fn read_file(path: &str) -> Vec<Vec<i32>> {
    let mut matrix: Vec<Vec<i32>> = Vec::new();

    let file = File::open(path);
    let reader = BufReader::new(file.unwrap());
    for line in reader.lines() {
        let numbers: Vec<i32> = line.unwrap()
            .split_whitespace()
            .map(|s| s.parse::<i32>().unwrap())
            .collect();
        matrix.push(numbers);
    }
    matrix
}

fn is_safe(row: &Vec<i32>) -> bool {
    let mut is_increasing: bool = true;

    for i in 0..row.len() -1 {
        if row[i] > row[i+1] && i==0 { is_increasing = false; }

        let distance: i32 = (row[i] - row[i+1]).abs();
        if (row[i] < row[i+1] && !is_increasing)
            || (row[i] > row[i+1] && is_increasing)
            || (row[i] == row[i+1])
            || distance > 3{return false;}
    }
    true
}


fn is_safe_removal(row: &Vec<i32>) -> bool {
    if is_safe(row) {return true;}

    for i in 0..row.len() {
        let mut row_copy: Vec<i32> = row.clone();
        row_copy.remove(i);
        if is_safe(&row_copy) {return true;}
    }
    false
}

fn day2(matrix: &Vec<Vec<i32>>, removal: bool) -> i32 {
    let mut sum = 0;
    for i in 0..matrix.len() {
        if removal {
            if is_safe_removal(&matrix[i]) {sum += 1;}
        }else {
            if is_safe(&matrix[i]) { sum += 1; }
        }

    }
    sum
}


fn main() {
    let matrix: Vec<Vec<i32>> = read_file("data/day2.txt");
    println!("part1: {}", day2(&matrix, false));
    println!("part2: {}", day2(&matrix, true));

}