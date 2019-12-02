use std::fs;

use anyhow::{Context, Result};

fn read_input() -> Result<Vec<usize>> {
    let contents = fs::read_to_string("input/day2.txt")?;
    contents
        .split(',')
        .map(|s| Ok(s.parse::<usize>()?))
        .collect()
}

fn run(noun: usize, verb: usize, data: &[usize]) -> Option<Vec<usize>> {
    let mut program = data.to_owned();
    program[1] = noun;
    program[2] = verb;

    for index in (0..program.len()).step_by(4) {
        let opcode = program[index];

        let read_pos_a = program[index + 1];
        let read_pos_b = program[index + 2];
        let out_pos = program[index + 3];

        match opcode {
            1 => program[out_pos] = program[read_pos_a] + program[read_pos_b],
            2 => program[out_pos] = program[read_pos_a] * program[read_pos_b],
            99 => break,
            _ => return None,
        }
    }

    Some(program)
}

#[test]
fn test_run() {
    let test = vec![1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50];
    let result = run(9, 10, &test);
    assert_eq!(
        vec!(3500, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50),
        result.unwrap()
    );
}

fn solve(noun: usize, verb: usize, data: &[usize]) -> Option<usize> {
    run(noun, verb, data).map(|program| program[0])
}

fn main() -> Result<()> {
    let input = read_input()?;

    let part1 = solve(12, 2, &input).context("failed to find a solution")?;
    println!("part1: {}", part1);

    let val = 19_690_720;
    for noun in 0..100 {
        for verb in 0..100 {
            let result = solve(noun, verb, &input);
            if result == Some(val) {
                println!("part2: {}", 100 * noun + verb);
                return Ok(());
            }
        }
    }

    Ok(())
}
