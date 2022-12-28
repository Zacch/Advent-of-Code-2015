use std::fs;
use crate::traits::StringExtensions;

pub fn run() {
    println!("Day 16");
    let contents = fs::read_to_string("input/day_16.txt").unwrap();

    let mut sues: Vec<Sue> = vec![];

    for line in contents.lines() {
        let tokens = line.tokens();
        let numbers = line.to_int_vector();
        let mut sue = Sue { number: numbers[0], ..Default::default() };

        for i in 1..tokens.len() / 2 {
            match tokens[i * 2] {
                "children:" => sue.children = Some(numbers[i]),
                "cats:" => sue.cats = Some(numbers[i]),
                "samoyeds:" => sue.samoyeds = Some(numbers[i]),
                "pomeranians:" => sue.pomeranians = Some(numbers[i]),
                "akitas:" => sue.akitas = Some(numbers[i]),
                "vizslas:" => sue.vizslas = Some(numbers[i]),
                "goldfish:" => sue.goldfish = Some(numbers[i]),
                "trees:" => sue.trees = Some(numbers[i]),
                "cars:" => sue.cars = Some(numbers[i]),
                "perfumes:" => sue.perfumes = Some(numbers[i]),
                &_ => panic!()
            }
        }
        if test_sue(&sue) {
            println!("Part 1: {}", sue.number);
        }
        sues.push(sue);
    }
    for sue in &sues {
        if test2_sue(sue) {
            println!("Part 2: {}", sue.number);
        }
    }
}

fn test_sue(sue: &Sue) -> bool {
    if sue.children != None && sue.children != Some(3) { return false; }
    if sue.cats != None && sue.cats!= Some(7) { return false; }
    if sue.samoyeds != None && sue.samoyeds != Some(2) { return false; }
    if sue.pomeranians != None && sue.pomeranians != Some(3) { return false; }
    if sue.akitas != None && sue.akitas != Some(0) { return false; }
    if sue.vizslas != None && sue.vizslas != Some(0) { return false; }
    if sue.goldfish != None && sue.goldfish != Some(5) { return false; }
    if sue.trees != None && sue.trees != Some(3) { return false; }
    if sue.cars != None && sue.cars != Some(2) { return false; }
    if sue.perfumes != None && sue.perfumes != Some(1) { return false; }
    true
}
fn test2_sue(sue: &Sue) -> bool {
    if sue.children != None && sue.children != Some(3) { return false; }
    if sue.cats != None && sue.cats <= Some(7) { return false; }
    if sue.samoyeds != None && sue.samoyeds != Some(2) { return false; }
    if sue.pomeranians != None && sue.pomeranians >= Some(3) { return false; }
    if sue.akitas != None && sue.akitas != Some(0) { return false; }
    if sue.vizslas != None && sue.vizslas != Some(0) { return false; }
    if sue.goldfish != None && sue.goldfish >= Some(5) { return false; }
    if sue.trees != None && sue.trees <= Some(3) { return false; }
    if sue.cars != None && sue.cars != Some(2) { return false; }
    if sue.perfumes != None && sue.perfumes != Some(1) { return false; }
    true
}

#[derive(Default, Debug)]
struct Sue {
    number: i32,
    children: Option<i32>,
    cats: Option<i32>,
    samoyeds: Option<i32>,
    pomeranians: Option<i32>,
    akitas: Option<i32>,
    vizslas: Option<i32>,
    goldfish: Option<i32>,
    trees: Option<i32>,
    cars: Option<i32>,
    perfumes: Option<i32>,
}

