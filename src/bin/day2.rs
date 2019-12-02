use std::fs;
use std::io::{BufRead, BufReader};

use anyhow::Result;

fn read_input() -> Result<Vec<usize>> {
    let contents = fs::read_to_string("input/day2.txt")?;
    contents
        .split(",")
        .map(|s| Ok(s.parse::<usize>()?))
        .collect()
}

fn run(noun: usize, verb: usize, data: &Vec<usize>) -> usize {
    let mut program = data.clone();
    program[1] = noun;
    program[2] = verb;

    for index in (0..program.len()).step_by(4) {
        let opcode = program[index];

        let read_pos_a = program[index + 1];
        let read_pos_b = program[index + 2];
        let out_pos = program[index + 3];

        if opcode == 1 {
            program[out_pos] = program[read_pos_a] + program[read_pos_b];
        } else if opcode == 2 {
            program[out_pos] = program[read_pos_a] * program[read_pos_b];
        } else if opcode == 99 {
            break;
        }
    }

    return program[0];
}

fn main() -> Result<()> {
    let input = read_input()?;
    let part1 = run(12, 2, &input);
    println!("part1: {}", part1);

    let val = 19690720;
    for noun in 0..99 {
        for verb in 0..99 {
            let result = run(noun, verb, &input);
            if result == val {
                println!("part2: {}", 100 * noun + verb);
                return Ok(());
            }
        }
    }

    Ok(())
}
