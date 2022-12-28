use std::fs;
use crate::two_d::{Point, Rect};

pub fn run() {
    println!("Day 18");
    let contents = fs::read_to_string("input/day_18.txt").unwrap();

    let mut lights: Vec<Vec<bool>> = vec![];
    for line in contents.lines() {
        lights.push(line.chars().map(|c| c == '#').collect())
    }
    let bounds = Rect { left: 0, top: lights.len() as i32, bottom: 0, right: lights[0].len() as i32 };

    let mut lights2 = lights.to_vec();
    light_corners(bounds, &mut lights2);

    for _ in 1..=100 { lights = run_step(&lights, bounds); }
    println!("Part 1: {}", count_lit_lights(&mut lights));

    for _ in 1..=100 {
        lights2 = run_step(&lights2, bounds);
        light_corners(bounds, &mut lights2);
    }
    println!("Part 2: {}", count_lit_lights(&lights2));
}

fn light_corners(bounds: Rect, lights: &mut Vec<Vec<bool>>) {
    lights[0][0] = true;
    lights[0][bounds.top as usize - 1] = true;
    lights[bounds.right as usize - 1][0] = true;
    lights[bounds.right as usize - 1][bounds.top as usize - 1] = true;
}

fn run_step(lights: &Vec<Vec<bool>>, bounds: Rect) -> Vec<Vec<bool>> {
    let mut new_lights: Vec<Vec<bool>> = vec![];
    for y in bounds.bottom..bounds.top {
        let mut new_row: Vec<bool> = vec![];
        for x in bounds.left..bounds.right {
            let point = Point { x, y };
            let neighbours = point.neighbours();
            let lit_neighbours: usize = neighbours.iter()
                .filter(|n| bounds.contains(**n) && lights[n.y as usize][n.x as usize])
                .count();
            new_row.push(match lights[y as usize][x as usize] {
                true => { lit_neighbours == 2 || lit_neighbours == 3 }
                false => { lit_neighbours == 3 }
            })
        }
        new_lights.push(new_row);
    }
    new_lights
}

fn count_lit_lights(lights: &Vec<Vec<bool>>) -> usize {
    lights.iter().map(|v| v.iter().filter(|i| **i).count()).sum()
}
