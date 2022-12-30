use std::fs;
use crate::traits::StringExtensions;

use crate::day_23::Register::*;
use crate::day_23::OpCode::*;

pub fn run() {
    println!("Day 23");
    let contents = fs::read_to_string("input/day_23.txt").unwrap();

    let mut program: Vec<Instruction> = vec![];

    for line in contents.lines() {
        let tokens = line.tokens();
        let register = match tokens[1] {
            "a" | "a," => A,
            "b" | "b," => B,
            _ => Null
        };
        let mut value = 0;
        if register == Null { value = tokens[1].parse().unwrap(); }
        if tokens.len() == 3 { value = tokens[2].parse().unwrap(); }

        program.push(
            match tokens[0] {
                "hlf" => { Instruction { op_code: Half, register, value, } },
                "tpl" => { Instruction { op_code: Triple, register, value, } },
                "inc" => { Instruction { op_code: Increment, register, value, } },
                "jmp" => { Instruction { op_code: Jump, register, value, } },
                "jie" => { Instruction { op_code: JumpIfEven, register, value, } },
                "jio" => { Instruction { op_code: JumpIfOne, register, value, } },
                _ => panic!()
            });
    }

    let (_a, b) = execute(&program, (0, 0));
    println!("Part 1: {}", b);

    let (_a2, b2) = execute(&program, (1, 0));
    println!("Part 2: {}", b2);
}

fn execute(program: &Vec<Instruction>, registers: (i64, i64)) -> (i64, i64) {

    let mut a = registers.0;
    let mut b = registers.1;
    let mut ip: i64 = 0;

    while ip >= 0 && ip < program.len() as i64 {
        let instruction = program[ip as usize];
        ip += 1;
        match instruction.op_code {
            Half => { match instruction.register { A => a /= 2, B => b /= 2, Null => panic!()}}
            Triple => { match instruction.register { A => a *= 3, B => b *= 3, Null => panic!()}}
            Increment => { match instruction.register { A => a += 1, B => b += 1, Null => panic!()}}
            Jump => { ip += instruction.value - 1; }
            JumpIfEven => {
                match instruction.register {
                    A => if a % 2 == 0 { ip += instruction.value - 1; },
                    B => if b % 2 == 0 { ip += instruction.value - 1; },
                    Null => panic!()
                }
            }
            JumpIfOne =>  {
                match instruction.register {
                    A => if a == 1 { ip += instruction.value - 1; },
                    B => if b == 1 { ip += instruction.value - 1; },
                    Null => panic!()
                }
            }
        }
    }
    (a, b)
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Ord, PartialOrd)]
enum OpCode { Half, Triple, Increment, Jump, JumpIfEven, JumpIfOne }

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Ord, PartialOrd)]
enum Register { A, B, Null }

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Ord, PartialOrd)]
struct Instruction {
    op_code: OpCode,
    register: Register,
    value: i64
}
