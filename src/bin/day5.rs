use std::fs;
use std::collections::{HashMap, HashSet};

fn read_list(content: &str, split_by: char) -> Vec<Vec<i32>> {
    content
        .lines()
        .map(|l| {
            l
                .split(split_by)
                .map(|l| l
                    .parse::<i32>()
                    .unwrap())
            .collect()
        }).collect()
}


fn read_file(path: &str) -> (Vec<Vec<i32>>, Vec<Vec<i32>>) {
    let file_content = fs::read_to_string(path).unwrap();

    let mut parts = file_content.split("\n\n");
    let first_part = parts.next().unwrap_or("").trim();
    let second_part = parts.next().unwrap_or("").trim();
    let order = read_list(first_part, '|');
    let pages = read_list(second_part, ',');
    (order, pages)
}


fn create_graph(order: Vec<Vec<i32>>) -> HashMap<i32, HashSet<i32>> {
    let mut graph: HashMap<i32, HashSet<i32>> = HashMap::new();

    for rule in order {
        graph.entry(rule[0]).or_insert(HashSet::new()).insert(rule[1]);
    }
    graph
}

fn check_order(graph: &HashMap<i32, HashSet<i32>>, page: &Vec<i32>) -> bool {

    for i in 0..page.len() -1 {
        if let Some(value) = graph.get(&page[i]) {
            if !value.contains(&page[i+1]) {
                return false;
            }
        }else{
            return false;
        }

    }
    true
}


fn part1(order: &Vec<Vec<i32>>, pages: &Vec<Vec<i32>>) -> i32 {
    let mut sum = 0;

    let graph = create_graph(order.clone());

    for page in pages {
        if check_order(&graph, &page) {
            let middle = page.len() / 2;
            sum += page[middle];
        }
    }
    sum
}

fn order_page(graph: &HashMap<i32, HashSet<i32>>, page: &Vec<i32>) -> Vec<i32> {
    let mut input = page.clone();

    for i in 1..input.len() {
        let x = input[i];
        let mut j = i;

        while j > 0{
            let previous = input[j-1];
            if let Some(val) = graph.get(&x){
                if val.contains(&previous) {
                    input[j] = previous;
                    j -= 1;
                }else { break; }
            }else { break; }
        }
        input[j] = x;
    }

    input
}

fn part2(order: &Vec<Vec<i32>>, pages: &Vec<Vec<i32>>) -> i32 {
    let mut sum = 0;

    let graph = create_graph(order.clone());
    for page in pages {
        if !check_order(&graph, &page) {
            let new_page = order_page(&graph, &page);
            let middle = new_page.len() / 2;
            sum += new_page[middle];
        }
    }
    sum
}


fn main() {
    let (order, pages) = read_file("data/day5.txt");
    println!("Part 1: {}", part1(&order, &pages));
    println!("Part 2: {}", part2(&order, &pages));
}