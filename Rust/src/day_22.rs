use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashSet};
use std::fs;
use crate::day_21::Character;
use crate::traits::StringExtensions;

pub fn run() {
    println!("Day 22");
    let contents = fs::read_to_string("input/day_22.txt").unwrap();

    let numbers = contents.to_int_vector();
    let boss = Character { hit_points: numbers[0], damage: numbers[1], armor: 0 };

    println!("Part 1: {}", fight(&boss, false));
    println!("Part 2: {}", fight(&boss, true));
}

fn fight(boss: &Character, hard_mode: bool) -> i32 {

    let mut frontier: BinaryHeap<State> = BinaryHeap::new();
    frontier.push(State {
        cost_so_far: 0,
        player_mana: 500,
        player_hp: 50,
        boss_hp: boss.hit_points,
        shield_duration: 0,
        poison_duration: 0,
        recharge_duration: 0,
    });

    let mut visited: HashSet<State> = HashSet::new();

    while !frontier.is_empty() {
        let current = frontier.pop().unwrap();
        if current.boss_hp <= 0 {
            return current.cost_so_far;
        }
        if visited.contains(&current) { continue; }
        for neighbour in neighbours(current, boss.damage, hard_mode) {
            if neighbour.player_hp <= 0 { continue; }
            frontier.push(neighbour);
        }
        visited.insert(current);
    }
    panic!()
}

fn neighbours(current: State, boss_damage: i32, hard_mode: bool) -> Vec<State> {
    let mut state = current.clone();

    if hard_mode { state.player_hp -= 1; }
    if state.poison_duration > 0 {
        state.boss_hp -= 3;
        state.poison_duration -= 1;
    }
    if state.recharge_duration > 0 {
        state.player_mana += 101;
        state.recharge_duration -= 1;
    }
    if state.shield_duration > 0 {
        state.shield_duration -= 1;
    }
    let mut neighbours: Vec<State> = vec![];

    if state.player_mana >= 53 {
        neighbours.push(State {
            cost_so_far: state.cost_so_far + 53,
            player_mana: state.player_mana - 53,
            boss_hp: state.boss_hp - 4,
            ..state
        });
    }
    if state.player_mana >= 73 {
        neighbours.push(State {
            cost_so_far: state.cost_so_far + 73,
            player_mana: state.player_mana - 73,
            player_hp: state.player_hp + 2,
            boss_hp: state.boss_hp - 2,
            ..state
        });
    }
    if state.player_mana >= 113 && state.shield_duration == 0 {
        neighbours.push(State {
            cost_so_far: state.cost_so_far + 113,
            player_mana: state.player_mana - 113,
            shield_duration: 6,
            ..state
        });
    }

    if state.player_mana >= 173 && state.poison_duration == 0 {
        neighbours.push(State {
            cost_so_far: state.cost_so_far + 173,
            player_mana: state.player_mana - 173,
            poison_duration: 6,
            ..state });
    }

    if state.player_mana >= 229 && state.recharge_duration == 0 {
        neighbours.push(State {
            cost_so_far: state.cost_so_far + 229,
            player_mana: state.player_mana - 229,
            recharge_duration: 5,
            ..state });
    }

    for neighbour in neighbours.iter_mut() {
        if neighbour.poison_duration > 0 {
            neighbour.boss_hp -= 3;
            neighbour.poison_duration -= 1;
        }
        if neighbour.recharge_duration > 0 {
            neighbour.player_mana += 101;
            neighbour.recharge_duration -= 1;
        }
        if neighbour.boss_hp > 0 {
            if neighbour.shield_duration > 0 {
                neighbour.player_hp -= boss_damage - 7;
                neighbour.shield_duration -= 1;
            } else {
                neighbour.player_hp -= boss_damage;
            }
        }
    }
    neighbours
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
struct State {
    cost_so_far: i32,
    player_mana: i32,
    player_hp: i32,
    boss_hp: i32,
    shield_duration: i32,
    poison_duration: i32,
    recharge_duration: i32,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost_so_far.cmp(&self.cost_so_far)
            .then_with(|| other.boss_hp.cmp(&self.boss_hp))
            .then_with(|| other.player_hp.cmp(&self.player_hp))
            .then_with(|| other.player_mana.cmp(&self.player_mana))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
