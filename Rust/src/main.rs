use std::time::Instant;
use advent_of_code_2015::*;

fn main() {
    let now = Instant::now();

    println!("Hello, world!");
    println!("Execution time: {:?}", Instant::now().checked_duration_since(now).unwrap());

}
