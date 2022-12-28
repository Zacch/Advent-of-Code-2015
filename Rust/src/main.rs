use std::time::Instant;
use advent_of_code_2015::*;

fn main() {
    let now = Instant::now();

    day_18::run();

    println!("Execution time: {:?}", Instant::now().checked_duration_since(now).unwrap());

}
