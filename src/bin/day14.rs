use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};
use AdventOfCode::board::Coord;

#[derive(Debug, Clone)]
struct Robot {
    position: Coord,
    movement: Coord,
}

enum Quadrants {
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
}

const WIDTH: i32 = 101;
const HEIGHT: i32 = 103;
fn read_file(path: &str) -> HashMap<i32, Robot> {
    let file = File::open(path);
    let reader = BufReader::new(file.unwrap());
    let re = Regex::new(r"-?\d+").unwrap();

    reader
        .lines()
        .enumerate()
        .filter_map(|(id, l)| {
            let line = l.unwrap();
            let numbers: Vec<i32> = re
                .find_iter(&line)
                .filter_map(|d| d.as_str().parse::<i32>().ok())
                .collect();
            if numbers.len() == 4 {
                Some((
                    id as i32,
                    Robot {
                        position: Coord {
                            x: numbers[0],
                            y: numbers[1],
                        },
                        movement: Coord {
                            x: numbers[2],
                            y: numbers[3],
                        },
                    },
                ))
            } else {
                None
            }
        })
        .collect::<HashMap<i32, Robot>>()
}

fn print_matrix(robots: &HashMap<i32, Robot>) {
    let mut matrix = vec![vec!['.'; WIDTH as usize]; HEIGHT as usize];

    for robot in robots.values() {
        let x = robot.position.x as usize;
        let y = robot.position.y as usize;
        if matrix[y][x] == '.' {
            matrix[y][x] = '1';
        } else {
            let count = matrix[y][x].to_digit(10).unwrap() + 1;
            matrix[y][x] = std::char::from_digit(count, 10).unwrap();
        }
    }

    for row in matrix {
        println!("{}", row.iter().collect::<String>());
    }
}

fn generate_first_row(robots: &HashMap<i32, Robot>) -> Vec<char> {
    let mut row = vec!['.'; WIDTH as usize];

    for robot in robots.values() {
        if robot.position.y == 0 {
            let x = robot.position.x as usize;
            if row[x] == '.' {
                row[x] = '1';
            } else {
                let count = row[x].to_digit(10).unwrap() + 1;
                row[x] = std::char::from_digit(count, 10).unwrap();
            }
        }
    }
    row
}

fn do_move(robot: &Robot) -> Robot {
    let mut new_position = Coord {
        x: robot.position.x + robot.movement.x,
        y: robot.position.y + robot.movement.y,
    };

    new_position = Coord {
        x: if new_position.x < 0 {
            WIDTH + (new_position.x % WIDTH)
        } else if new_position.x >= WIDTH {
            new_position.x % WIDTH
        } else {
            new_position.x
        },
        y: if new_position.y < 0 {
            HEIGHT + (new_position.y % HEIGHT)
        } else if new_position.y >= HEIGHT {
            new_position.y % HEIGHT
        } else {
            new_position.y
        },
    };

    Robot {
        position: new_position,
        movement: robot.movement,
    }
}

fn move_robots(robots: &HashMap<i32, Robot>) -> HashMap<i32, Robot> {
    let mut new_robots = robots.clone();
    for (id, robot) in robots {
        new_robots.insert(*id, do_move(&robot));
    }
    new_robots
}

fn check_quadrant(robot: &Robot) -> Option<Quadrants> {
    let in_left = robot.position.x < WIDTH / 2;
    let in_top = robot.position.y < HEIGHT / 2;
    let in_right = robot.position.x > WIDTH / 2;
    let in_bottom = robot.position.y > HEIGHT / 2;
    match (in_left, in_top, in_right, in_bottom) {
        (true, true, false, false) => Some(Quadrants::TopLeft),
        (false, true, true, false) => Some(Quadrants::TopRight),
        (true, false, false, true) => Some(Quadrants::BottomLeft),
        (false, false, true, true) => Some(Quadrants::BottomRight),
        _ => None,
    }
}

fn part1(robots: &HashMap<i32, Robot>, seconds: i32) -> i32 {
    let mut robots = (0..seconds).fold(robots.clone(), |curr_robot, _| move_robots(&curr_robot));

    robots
        .values()
        .map(|robot| match check_quadrant(&robot) {
            Some(q) => match q {
                Quadrants::TopLeft => (1, 0, 0, 0),
                Quadrants::TopRight => (0, 1, 0, 0),
                Quadrants::BottomLeft => (0, 0, 1, 0),
                Quadrants::BottomRight => (0, 0, 0, 1),
            },
            None => (0, 0, 0, 0),
        })
        .reduce(|acc, quadrant| {
            (
                acc.0 + quadrant.0,
                acc.1 + quadrant.1,
                acc.2 + quadrant.2,
                acc.3 + quadrant.3,
            )
        })
        .map(|(q1, q2, q3, q4)| q1 * q2 * q3 * q4)
        .unwrap_or(0)
}

fn find_pattern(robots: &HashMap<i32, Robot>) -> bool {
    let mut rows: HashMap<i32, Vec<i32>> = HashMap::new();

    for robot in robots.values() {
        rows.entry(robot.position.y)
            .or_insert_with(Vec::new)
            .push(robot.position.x);
    }

    for (_, cols) in rows {
        let mut sorted_cols = cols.clone();
        sorted_cols.sort_unstable();

        let mut count = 1;
        for i in 1..sorted_cols.len() {
            if sorted_cols[i] == sorted_cols[i - 1] + 1 {
                count += 1;
                if count == 31 {
                    return true;
                }
            } else {
                count = 1;
            }
        }
    }
    false
}

fn part2(robots: &HashMap<i32, Robot>) -> i32 {
    let mut robots = robots.clone();
    let mut count =0;
    loop {
        count +=1;
        robots = move_robots(&robots);
        if find_pattern(&robots) {
            println!();
            print_matrix(&robots);
            println!();
            return count;
        }
    }
}

fn main() {
    let robots = read_file("data/day14.txt");
    println!("Part 1: {}", part1(&robots, 100));
    println!("Part 2: {}", part2(&robots))
}
