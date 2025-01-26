use crate::board::{Board, Coord};
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::hash::Hash;

pub trait StateTrait: Eq + PartialEq + Clone + Hash {
    fn get_coord(&self) -> Coord;
}

pub trait NodeTrait {
    type State: StateTrait;

    fn get_state(&self) -> Self::State;
    fn get_cost(&self) -> usize;
    fn get_h_cost(&self) -> usize;

    fn new(state: Self::State, cost: usize, h_cost: usize) -> Self;
}

pub fn a_star<S, N, F>(
    board: &Board<char>,
    start: S,
    end: S,
    get_neighbors: F,
    one_path: bool,
) -> (usize, HashMap<N, HashSet<N>>)
where
    S: StateTrait,
    N: NodeTrait<State = S> + Ord + Eq + PartialEq + Clone + Hash,
    F: Fn(&N, &S) -> Vec<N>,
{
    let mut min_score = usize::MAX;
    let mut paths: HashMap<N, HashSet<N>> = HashMap::new();
    let mut open_queue = BinaryHeap::new();
    let mut g_scores: HashMap<S, usize> = HashMap::new();

    g_scores.insert(start.clone(), 0);
    open_queue.push(N::new(start.clone(), 0, 0));

    while let Some(current_node) = open_queue.pop() {
        let current_coord = current_node.get_state().get_coord();

        if current_coord == end.get_coord() {
            if min_score > current_node.get_cost() {
                min_score = current_node.get_cost();
            }
            if one_path {
                return (min_score, paths);
            }
            continue;
        }

        let neighbors = get_neighbors(&current_node, &end);
        for neighbor_node in neighbors {
            let value = board.get_value(neighbor_node.get_state().get_coord());
            if !matches!(value, None | Some('#')) {
                let score = *g_scores
                    .get(&neighbor_node.get_state())
                    .unwrap_or(&usize::MAX);

                if neighbor_node.get_cost() < score {
                    paths.insert(neighbor_node.clone(), HashSet::from([current_node.clone()]));
                    g_scores.insert(neighbor_node.get_state(), neighbor_node.get_cost());

                    open_queue.push(neighbor_node);
                } else if neighbor_node.get_cost() == score {
                    paths
                        .entry(neighbor_node.clone())
                        .or_insert_with(|| HashSet::new())
                        .insert(current_node.clone());
                }
            }
        }
    }

    (min_score, paths)
}
