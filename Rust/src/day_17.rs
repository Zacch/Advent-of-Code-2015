use std::fs;

pub fn run() {
    println!("Day 17");
    let contents = fs::read_to_string("input/day_17.txt").unwrap();

    let mut containers: Vec<(usize, i32)> = vec![];
    for (row, line) in contents.lines().enumerate() {
        containers.push((row, line.parse().unwrap()));
    }

    let combinations = find_combinations(&containers, 150);

    let mut filtered_combinations: Vec<Vec<usize>> = vec![];
    for combination in combinations {
        if !filtered_combinations.iter()
            .any(|v|combination.len() == v.len() &&
                v.iter().all(|i| combination.contains(i))) {
            filtered_combinations.push(combination);
        }
    }
    println!("Part 1:{}", filtered_combinations.len());

    let minimum = filtered_combinations.iter().map(|i|i.len()).min().unwrap();
    filtered_combinations.retain(|v| v.len() == minimum);
    println!("Part 2:{}", filtered_combinations.len());
}

fn find_combinations(containers: &Vec<(usize, i32)>, amount: i32) -> Vec<Vec<usize>> {
    let mut result:Vec<Vec<usize>> = vec![];
    for (i, (row, size)) in containers.iter().enumerate() {
        if *size == amount {
            result.push(vec![*row]);
            continue;
        }

        let mut other_containers = containers.to_vec();
        other_containers.remove(i);
        other_containers.retain(|i| i.1 <= amount - *size);

        let mut sub_combinations =
            find_combinations(&other_containers, amount - *size);
        for sub_combination in sub_combinations.iter_mut() {
            sub_combination.push(*row);
            result.push(sub_combination.to_vec());
        }
    }
    result
}

