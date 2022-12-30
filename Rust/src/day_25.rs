use std::fs;
use crate::traits::StringExtensions;

pub fn run() {
    println!("Day 25");
    let contents = fs::read_to_string("input/day_25.txt").unwrap();
    let numbers = contents.to_int_vector();

    println!("{:?}", code_at(numbers[0] as u64, numbers[1] as u64));
}

fn code_at(row: u64, col: u64) -> u64 {
    let seed = 20151125;
    let base = 252533;
    let modulus = 33554393;

    let n = row + col - 2;
    let exponent = col + n * (n + 1) / 2 - 1;

    seed * mod_power(base, exponent, modulus) % modulus
}

fn mod_power(base: u64, exponent: u64, modulus: u64) -> u64 {
    if exponent == 0 { return 1; }
    if exponent == 1 { return base % modulus; }
    if base < 2 { return base; }

    let product = base * base % modulus;

    if exponent % 2 == 1 { return base * mod_power(product, exponent / 2, modulus) % modulus; }
    mod_power(product, exponent / 2, modulus)
}
