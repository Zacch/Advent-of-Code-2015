use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::fs;
use substring::*;
use crate::traits::StringExtensions;

pub fn run() {
    println!("Day 19");
    let contents = fs::read_to_string("input/day_19.txt").unwrap();

    let mut rules: HashMap<String, Vec<String>> = HashMap::new();
    let mut molecule: String = "".to_string();
    for line in contents.lines() {
        let tokens = line.tokens();
        match tokens.len() {
            0 => {},
            1 => molecule = tokens[0].to_string(),
            3 => {
                let key = tokens[0].to_string();
                let replacement = tokens[2].to_string();
                if rules.contains_key( &key) {
                    rules.get_mut(&key).unwrap().push(replacement);
                } else {
                    rules.insert(key, vec![replacement]);
                }
            },
            _ => panic!()
        }
    }

    println!("Part 1: {}", neighbours(&rules, &molecule, molecule.len() as i32).len());
    println!("Part 2: {}", part2(&rules,&molecule));
}

fn neighbours(rules: &HashMap<String, Vec<String>>, molecule: &String, max_len: i32) -> HashSet<String> {
    let mut mutations: HashSet<String> = HashSet::new();
    for start in 0.max(molecule.len() as i32 - max_len) as usize..molecule.len() {
        for end in start + 1..=molecule.len() {
            let substring = molecule.substring(start, end);
            if rules.contains_key(substring) {
                for replacement in &rules[substring] {
                    mutations.insert(molecule.substring(0, start).to_string()
                        + &replacement + molecule.substring(end, molecule.len()));
                }
                break;
            }
        }
    }
    mutations
}

fn part2(rules: &HashMap<String, Vec<String>>, goal: &String) -> usize {
    let goal_length = goal.len();
    let mut visited: HashSet<State> = HashSet::new();
    let mut frontier = BinaryHeap::new();
    frontier.push(
        State { molecule: "e".to_string(), cost_so_far: 0, est_total_cost: goal.len() });

    while !frontier.is_empty() {
        let current = frontier.pop().unwrap();
        if current.molecule == *goal { return current.cost_so_far; }

        if visited.contains(&current) { continue; }
        for neighbor in neighbours(rules, &current.molecule, 15) {
            let neighbour_len = neighbor.len();
            if neighbour_len > goal_length { continue; }
            if neighbour_len == goal_length && neighbor != *goal { continue; }

            let matching = neighbor.chars().zip(goal.substring(0, neighbour_len).chars()).
                filter(|&(a, b)| a == b).count();
            let diffs = neighbour_len - matching;

            let neighbour_state = State {
                molecule: neighbor.to_string(),
                cost_so_far: current.cost_so_far + 1,
                est_total_cost: current.cost_so_far + 1 + diffs * 50 + (goal_length - neighbour_len),
            };
            if visited.contains(&neighbour_state) { continue; }
            frontier.push(neighbour_state);
        }
        visited.insert(current);
    }
    panic!("No solution found")
}

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
struct State {
    molecule: String,
    cost_so_far: usize,
    est_total_cost: usize,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.est_total_cost.cmp(&self.est_total_cost)
            .then_with(|| other.molecule.cmp(&self.molecule))
    }
}
impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
