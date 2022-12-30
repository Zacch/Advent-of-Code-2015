use std::fs;
use crate::traits::StringExtensions;

pub fn run() {
    println!("Day 24");
    let contents = fs::read_to_string("input/day_24.txt").unwrap();

    let packages = contents.to_int_vector();
    let total_weight: i32 = packages.iter().sum::<i32>();

    println!("Part 1: {}", divide_packages(&packages, total_weight / 3, false));
    println!("Part 2: {}", divide_packages(&packages, total_weight / 4, true));
}

fn divide_packages(packages: &Vec<i32>, weight_per_group: i32, use_four_groups: bool) -> u64 {

    let mut smallest_group_1_size = packages.len() / 3;
    let mut lowest_quantum_entanglement = u64::MAX;

    let start_state = State { packages: packages.clone(), quantum_entanglement: 1, ..Default::default() };
    let mut frontier:Vec<State> = vec![start_state];

    while !frontier.is_empty() {
        let current = frontier.pop().unwrap();
        for neighbour in neighbours(&current, weight_per_group, use_four_groups) {
            let group_1_length = neighbour.group1.len();
            if group_1_length > smallest_group_1_size { continue; }
            if neighbour.quantum_entanglement >= lowest_quantum_entanglement { continue; }
            if neighbour.packages.is_empty() {
                if neighbour.is_balanced(use_four_groups) {
                    if group_1_length < smallest_group_1_size {
                        smallest_group_1_size = group_1_length;
                        lowest_quantum_entanglement = u64::MAX;
                    }
                    lowest_quantum_entanglement = lowest_quantum_entanglement.min(neighbour.quantum_entanglement);
                }
                continue;
            }
            frontier.push(neighbour);
        }
    }
    lowest_quantum_entanglement
}

fn neighbours(state: &State, max_weight: i32, use_four_groups: bool) -> Vec<State> {
    let mut neighbours: Vec<State> = vec![];
    let package = state.packages.last().unwrap();

    if state.weight1 + package <= max_weight {
        let mut neighbour = state.clone();
        neighbour.group1.push(neighbour.packages.pop().unwrap());
        neighbour.weight1 += package;
        neighbour.quantum_entanglement *= *package as u64;
        neighbours.push(neighbour)
    }
    if state.weight2 + package <= max_weight {
        let mut neighbour = state.clone();
        neighbour.group2.push(neighbour.packages.pop().unwrap());
        neighbour.weight2 += package;
        neighbours.push(neighbour)
    }
    if state.weight3 + package <= max_weight {
        let mut neighbour = state.clone();
        neighbour.group3.push(neighbour.packages.pop().unwrap());
        neighbour.weight3 += package;
        neighbours.push(neighbour)
    }
    if use_four_groups && state.weight4 + package <= max_weight {
        let mut neighbour = state.clone();
        neighbour.group4.push(neighbour.packages.pop().unwrap());
        neighbour.weight4 += package;
        neighbours.push(neighbour)
    }
    neighbours
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Ord, PartialOrd, Default)]
struct State {
    group1: Vec<i32>,
    weight1: i32,
    quantum_entanglement: u64,
    group2: Vec<i32>,
    weight2: i32,
    group3: Vec<i32>,
    weight3: i32,
    group4: Vec<i32>,
    weight4: i32,
    packages: Vec<i32>
}

impl State {
    fn is_balanced(&self, use_four_groups: bool) -> bool {
        if use_four_groups {
            self.packages.is_empty() && self.weight1 == self.weight2 &&
                self.weight1 == self.weight3 && self.weight1 == self.weight4
        } else {
            self.packages.is_empty() && self.weight1 == self.weight2 && self.weight1 == self.weight3
        }
    }
}
