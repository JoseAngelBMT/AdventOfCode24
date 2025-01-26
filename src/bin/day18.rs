use std::cmp::Ordering;
use std::fs::File;
use std::io::{BufRead, BufReader};
use AdventOfCode::board::{Board, Coord};
use AdventOfCode::path_finding::{a_star, NodeTrait, StateTrait};

const SIZE: usize = 71;

#[derive(Clone, Debug, Eq, PartialEq, Hash, Copy)]
struct State {
    coord: Coord,
}

impl State {
    fn new(coord: Coord) -> Self {
        Self { coord }
    }
}

impl StateTrait for State {
    fn get_coord(&self) -> Coord {
        self.coord.clone()
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Hash, Copy)]
struct Node {
    state: State,
    cost: usize,
    h_cost: usize,
}

impl Node {
    fn f_cost(&self) -> usize {
        self.cost + self.h_cost
    }
}

impl NodeTrait for Node {
    type State = State;
    fn get_state(&self) -> State {
        self.state.clone()
    }

    fn get_cost(&self) -> usize {
        self.cost
    }

    fn get_h_cost(&self) -> usize {
        self.h_cost
    }

    fn new(state: Self::State, cost: usize, h_cost: usize) -> Self {
        Self {
            state,
            cost,
            h_cost,
        }
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        other.f_cost().cmp(&self.f_cost())
    }
}

impl PartialOrd<Self> for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn read_input(path: &str) -> Vec<Coord> {
    let file = File::open(path);
    let reader = BufReader::new(file.unwrap());
    let coords = reader
        .lines()
        .filter_map(|line| {
            line.ok().and_then(|l| {
                l.split_once(',').and_then(|(x, y)| {
                    Some(Coord {
                        x: x.parse::<i32>().ok()?,
                        y: y.parse::<i32>().ok()?,
                    })
                })
            })
        })
        .collect();
    coords
}

fn add_coords(board: &mut Board<char>, coords: &[Coord]) {
    coords.iter().for_each(|coord| board.set_value(*coord, '#'))
}

fn heuristic(start: &Coord, end: &State) -> usize {
    ((end.coord.x - start.x).abs() + (end.coord.y - start.y).abs()) as usize
}

fn get_neighbors(node: &Node, end: &State) -> Vec<Node> {
    let coord = node.state.get_coord();
    let cost = node.cost;
    vec![
        Node::new(
            State::new(coord.up()),
            cost + 1,
            heuristic(&coord.up(), &end),
        ),
        Node::new(
            State::new(coord.down()),
            cost + 1,
            heuristic(&coord.down(), &end),
        ),
        Node::new(
            State::new(coord.right()),
            cost + 1,
            heuristic(&coord.right(), &end),
        ),
        Node::new(
            State::new(coord.left()),
            cost + 1,
            heuristic(&coord.left(), &end),
        ),
    ]
}

fn part1(board: &Board<char>, coords: &Vec<Coord>) -> usize {
    let mut board = board.clone();
    add_coords(&mut board, &coords[0..1024]);
    let (cost, _) = a_star(
        &board,
        State::new(Coord::new(0, 0)),
        State::new(Coord::new((SIZE - 1) as i32, (SIZE - 1) as i32)),
        get_neighbors,
        true,
    );
    cost
}

fn main() {
    let board = Board::<char>::empty(SIZE, SIZE);
    let coords = read_input("data/day18.txt");
    let tiles = part1(&board, &coords);
    println!("Part 1: {}", tiles);
}
