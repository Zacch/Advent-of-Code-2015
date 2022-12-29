use std::fs;

pub fn run() {
    println!("Day 20");
    let contents = fs::read_to_string("input/day_20.txt").unwrap();
    let target: usize = contents.trim().parse().unwrap();

    let mut part1 = 0;
    let mut part2 = 0;
    for house in 1..target {
        let (p1, p2) = presents_delivered(house);
        if part1 == 0 && p1 > target {
            part1 = house;
            if part2 > 0 { break; }
        }
        if part2 == 0 && p2 > target {
            part2 = house;
            if part1 > 0 { break; }
        }
    }

    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}

fn presents_delivered(house: usize) -> (usize, usize) {
    if house < 2 { return (house * 10, house * 11); }
    let mut p1 = 10 + house * 10;
    let mut p2 = house * 11;
    for elf in 2..=house / 2 {
        if house % elf == 0 {
            p1 += 10 * elf;
            if elf * 50 >= house { p2 += 11 * elf; }
        }
    }
    (p1, p2)
}
