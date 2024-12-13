use itertools::Itertools;
use std::fs::File;
use std::io::{BufRead, BufReader};

type Value = i64;
type Equation = (Value, Vec<Value>);

enum Operation {
    Sum,
    Product,
    Concatenate,
}

fn calculate(x: &Value, y: &Value, operation: &Operation) -> Value {
    match operation {
        Operation::Sum => x + y,
        Operation::Product => x * y,
        Operation::Concatenate => {
            let concatenate = format!("{}{}", x, y);
            concatenate.parse::<Value>().unwrap()
        }
    }
}


fn parse_equation(equation: &str) -> Equation {
    let (head, tail) = equation.split_once(":").unwrap();
    let row: Vec<Value> = tail
        .trim()
        .split_whitespace()
        .map(|val| val.parse::<Value>().unwrap())
        .collect();
    (head.parse::<Value>().unwrap(), row)
}

fn read_file(path: &str) -> Vec<Equation> {
    let file = File::open(path);
    let reader = BufReader::new(file.unwrap());

    let rows = reader
        .lines()
        .map(|line| parse_equation(&line.unwrap()))
        .collect_vec();
    rows
}

fn resolve_equation(target: &Value, elements: &mut Vec<Value>, operations: &Vec<Operation>) -> bool {
    if elements.len() == 1 {
        if elements[0] == *target {
            return true;
        }
        return false;
    }
    let mut result: bool = false;

    let x = elements[0];
    let y = elements[1];
    let mut vector = elements[2..].to_vec();

    for op in operations {
        let new_value = calculate(&x, &y, op);
        vector.insert(0, new_value);
        result = resolve_equation(target, &mut vector, &operations) || result;
        vector.remove(0);
    }
    result
}

fn resolve(equations: &Vec<Equation>, operations: &Vec<Operation>) -> Value {
    let mut sum = 0;
    for (result, values) in equations {
        let mut elements = values.clone();
        if resolve_equation(result, elements.as_mut(), operations) {
            sum += result;
        }
    }
    sum
}

fn main() {
    let equations = read_file("data/day7.txt");
    let mut operations = vec![Operation::Sum, Operation::Product];
    println!("Part 1: {}", resolve(&equations, &operations));
    operations.push(Operation::Concatenate);
    println!("Part 2: {}", resolve(&equations, &operations));
}
