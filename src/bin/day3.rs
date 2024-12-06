use regex::Regex;
use std::fs;

fn read_file(path: &str) -> Result<String, std::io::Error> {
    fs::read_to_string(path)
}

fn part1(data: &String) -> i32{
    let mut value: i32 = 0;
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    for cap in re.captures_iter(&data) {
        let x: i32 = cap[1].parse().unwrap();
        let y: i32 = cap[2].parse().unwrap();
        value += x*y
    }
    value
}

fn part2(data: &String) -> i32{
    let mut value: i32 = 0;
    let mut is_enable: bool = true;
    let re = Regex::new(r"(?<do>do\(\))|(?<don>don't\(\))|(?<mul>mul\((\d+),(\d+)\))").unwrap();
    for cap in re.captures_iter(&data) {
        if let Some(_do) = cap.name("do"){
            is_enable = true;
        }else if let Some(_don) = cap.name("don"){
            is_enable = false;
        }else if let Some(_mul) = cap.name("mul"){
            if is_enable{
                let x: i32 = cap[4].parse().unwrap();
                let y: i32 = cap[5].parse().unwrap();
                value += x*y
            }

        }

    }
    value
}

fn main() {
    match read_file("data/day3.txt") {
        Ok(data) => {
            println!("Part 1: {}", part1(&data));
            println!("Part 2: {}", part2(&data));
        },
        Err(e) => eprintln!("Error reading file: {}", e),
    }
}