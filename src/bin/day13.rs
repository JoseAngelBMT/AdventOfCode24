use regex::Regex;
use std::fs::File;
use std::io::{BufRead, BufReader};

type Value = i64;
fn read_file(path: &str) -> Vec<Vec<Value>> {
    let file = File::open(path);
    let reader = BufReader::new(file.unwrap());
    let re = Regex::new(r"\d+").unwrap();

    let lines = reader
        .lines()
        .filter_map(|l| {
            let line = l.unwrap();
            let captures: Vec<Value> = re
                .captures_iter(&line)
                .map(|c| c[0].parse::<Value>().unwrap())
                .collect();
            if captures.is_empty() {
                None
            } else {
                Some(captures)
            }
        })
        .collect();

    lines
}

fn solve_equations(xs: &Vec<Value>, ys: &Vec<Value>, res: &Vec<Value>) -> Option<(Value, Value)> {
    let (a1, b1, c1) = (xs[0], ys[0], res[0]);
    let (a2, b2, c2) = (xs[1], ys[1], res[1]);

    let det = a1 * b2 - a2 * b1;

    if det == 0 {
        return None;
    }

    // Calculate x and y using Cramer's Rule
    let x: f64 = (c1 * b2 - c2 * b1) as f64 / det as f64;
    let y: f64 = (a1 * c2 - a2 * c1) as f64 / det as f64;

    if x.fract() == 0.0 && y.fract() == 0.0 {
        Some((x as Value, y as Value))
    } else {
        None
    }
}

fn day13(movements: &Vec<Vec<Value>>, part2: bool) -> Value {
    let increment: Value = 10000000000000;
    let result = movements
        .chunks(3)
        .filter_map(|chunk| {
            if chunk.len() == 3 {
                if part2 {
                    let result: Vec<Value> = chunk[2].iter().map(|&x| x + increment).collect();
                    solve_equations(&chunk[0], &chunk[1], &result)
                } else {
                    solve_equations(&chunk[0], &chunk[1], &chunk[2])
                }
            } else {
                None
            }
        })
        .collect::<Vec<(Value, Value)>>();
    result.iter().map(|(x, y)| *x * 3 + *y).sum()
}

fn main() {
    let movements = read_file("data/day13.txt");
    println!("Part 1: {}", day13(&movements, false));
    println!("Part 2: {}", day13(&movements, true));
}
