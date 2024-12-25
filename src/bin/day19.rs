use std::collections::HashMap;

fn read_file(path: &str) -> (Vec<String>, Vec<String>) {
    let content = std::fs::read_to_string(path).expect("Failed to read file");
    let mut parts = content.split("\n\n");

    let sequence = parts.next().expect("Missing sequence");
    let designs: Vec<String> = sequence.split(",").map(|c| c.trim().to_string()).collect();

    let board_str = parts.next().expect("Missing Board part");
    let lines: Vec<String> = board_str
        .lines()
        .map(|line| line.trim().to_string())
        .collect();

    (designs, lines)
}

fn correct_design(designs: &Vec<String>, row: &String, memory: &mut HashMap<String, i64>) -> i64 {
    if let Some(&result) = memory.get(row) {
        return result;
    }

    if row.is_empty() {
        return 1;
    }

    let mut combinations = 0;

    for design in designs {
        if let Some(remaining) = row.strip_prefix(design) {
            combinations += correct_design(designs, &remaining.to_string(), memory);
        }
    }

    memory.insert(row.to_string(), combinations);
    combinations
}

fn part1(designs: &Vec<String>, lines: &Vec<String>) -> i32 {
    let mut memory = HashMap::new();
    lines
        .iter()
        .filter(|x| 0!=correct_design(designs, x, &mut memory))
        .count() as i32
}

fn part2(designs: &Vec<String>, lines: &Vec<String>) -> i64 {
    let mut memory = HashMap::new();
    lines
        .iter()
        .map(|x| correct_design(designs, x, &mut memory))
        .sum()
}

fn main() {
    let (designs, lines) = read_file("data/day19.txt");
    println!("Part 1: {}", part1(&designs, &lines));
    println!("Part 2: {}", part2(&designs, &lines));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day19_part1_base_case() {
        let (designs, lines) = read_file("data/day19test.txt");
        assert_eq!(part1(&designs, &lines), 6);
    }
    #[test]
    fn test_day19_part2_base_case() {
        let (designs, lines) = read_file("data/day19test.txt");
        assert_eq!(part2(&designs, &lines), 16);
    }

}
