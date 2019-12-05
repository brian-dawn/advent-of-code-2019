#[macro_use]
extern crate anyhow;

use std::convert::TryInto;
use std::fs;

use anyhow::{Context, Result};

#[derive(Debug, PartialEq, Eq)]
enum Parameter {
    PositionMode(i32),
    ImmediateMode(i32),
}

type Memory = Vec<i32>;

#[derive(Debug, PartialEq, Eq)]
struct OpCodeMode {
    opcode: u8,
    p1: u8,
    p2: u8,
    p3: u8,
}

impl From<i32> for OpCodeMode {
    fn from(code: i32) -> OpCodeMode {
        OpCodeMode {
            opcode: (code % 100) as u8,
            p1: ((code / 100) % 10) as u8,
            p2: ((code / 1000) % 10) as u8,
            p3: ((code / 10000) % 10) as u8,
        }
    }
}

#[test]
fn from_opcode_mode() {
    assert_eq!(
        OpCodeMode {
            opcode: 2,
            p1: 0,
            p2: 1,
            p3: 0
        },
        1002.into()
    );

    assert_eq!(
        OpCodeMode {
            opcode: 99,
            p1: 0,
            p2: 0,
            p3: 1
        },
        10099.into()
    );
}

impl Parameter {
    fn build(mode: u8, parameter: i32) -> Result<Parameter> {
        match mode {
            0 => Ok(Parameter::PositionMode(parameter)),
            1 => Ok(Parameter::ImmediateMode(parameter)),
            _ => Err(anyhow!("unknown mode {}", mode)),
        }
    }
    fn realize(self, memory: &Memory) -> Result<i32> {
        match self {
            Parameter::PositionMode(n) => {
                let u: usize = n.try_into()?;
                let out: &i32 = memory
                    .get(u)
                    .context(format!("failed to load memory address {}", n))?;
                Ok(*out)
            }
            Parameter::ImmediateMode(n) => Ok(n),
        }
    }
}

fn read_input() -> Result<Memory> {
    let contents = fs::read_to_string("input/day5.txt")?;
    contents
        .split(',')
        .map(|s| Ok(s.trim().parse::<i32>()?))
        .collect()
}

fn run_program(program: &mut Memory, input: i32) -> Result<i32> {
    let mut index = 0;
    let mut outputs = Vec::new();
    loop {
        let code = program.get(index).context("read failed")?;
        let modes: OpCodeMode = (*code).into();

        let raw1 = program.get(index + 1).map(|i| *i);
        let raw2 = program.get(index + 2).map(|i| *i);
        let raw3 = program.get(index + 3).map(|i| *i);

        let p1 = raw1.context("invalid program p1");
        let _p2 = raw2.context("invalid program p2");
        let p3 = raw3.context("invalid program p3");

        let real_p1 = raw1
            .context("invalid program realp1")
            .and_then(|i| Parameter::build(modes.p1, i)?.realize(program));
        let real_p2 = raw2
            .context("invalid program realp2")
            .and_then(|i| Parameter::build(modes.p2, i)?.realize(program));
        let _real_p3 = raw3
            .context("invalid program realp3")
            .and_then(|i| Parameter::build(modes.p3, i)?.realize(program));

        match modes.opcode {
            1 => {
                index += 4;
                let output_addr: usize = p3?.try_into()?;
                program[output_addr] = real_p1? + real_p2?;
            }

            2 => {
                index += 4;
                let output_addr: usize = p3?.try_into()?;
                program[output_addr] = real_p1? * real_p2?;
            }

            3 => {
                index += 2;
                let addr: usize = p1?.try_into()?;
                program[addr] = input;
            }

            4 => {
                index += 2;
                outputs.push(real_p1?);
            }
            5 => {
                index += 3;
                if real_p1? != 0 {
                    index = real_p2?.try_into()?;
                }
            }
            6 => {
                index += 3;
                if real_p1? == 0 {
                    index = real_p2?.try_into()?;
                }
            }
            7 => {
                index += 4;

                let output_addr: usize = p3?.try_into()?;
                program[output_addr] = if real_p1? < real_p2? { 1 } else { 0 };
            }

            8 => {
                index += 4;

                let output_addr: usize = p3?.try_into()?;
                program[output_addr] = if real_p1? == real_p2? { 1 } else { 0 };
            }
            99 => break,
            _ => return Err(anyhow!("unknown opcode {}", modes.opcode)),
        };
    }

    outputs.last().context("no output").map(|i| *i)
}

#[test]
fn test_run_program() -> Result<()> {
    // Program test input == 8
    let mut program = vec![3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8];
    let mut result = run_program(&mut program, 8)?;
    assert_eq!(1, result);

    program = vec![3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8];
    result = run_program(&mut program, 3)?;
    assert_eq!(0, result);

    // Program test input < 8
    program = vec![3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8];
    result = run_program(&mut program, 4)?;
    assert_eq!(1, result);

    program = vec![3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8];
    result = run_program(&mut program, 99)?;
    assert_eq!(0, result);

    // Program test input == 8 immediate mode
    program = vec![3, 3, 1108, -1, 8, 3, 4, 3, 99];
    result = run_program(&mut program, 8)?;
    assert_eq!(1, result);

    program = vec![3, 3, 1108, -1, 8, 3, 4, 3, 99];
    result = run_program(&mut program, 10)?;
    assert_eq!(0, result);

    // Program test input < 8 immediate mode
    program = vec![3, 3, 1107, -1, 8, 3, 4, 3, 99];
    result = run_program(&mut program, 4)?;
    assert_eq!(1, result);

    program = vec![3, 3, 1107, -1, 8, 3, 4, 3, 99];
    result = run_program(&mut program, 99)?;
    assert_eq!(0, result);

    // Jump tests for zero.
    program = vec![3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, -1, 0, 1, 9];
    result = run_program(&mut program, 0)?;
    assert_eq!(0, result);

    program = vec![3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, -1, 0, 1, 9];
    result = run_program(&mut program, -1)?;
    assert_eq!(1, result);

    // Jump tests for zero immediate mode
    program = vec![3, 3, 1105, -1, 9, 1101, 0, 0, 12, 4, 12, 99, 1];
    result = run_program(&mut program, 0)?;
    assert_eq!(0, result);

    program = vec![3, 3, 1105, -1, 9, 1101, 0, 0, 12, 4, 12, 99, 1];
    result = run_program(&mut program, -1)?;
    assert_eq!(1, result);

    // Long example
    let long = vec![
        3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31, 1106, 0, 36, 98, 0, 0,
        1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104, 999, 1105, 1, 46, 1101, 1000, 1, 20, 4, 20,
        1105, 1, 46, 98, 99,
    ];

    let mut a = long.clone();
    result = run_program(&mut a, 7)?;
    assert_eq!(999, result);

    let mut b = long.clone();
    result = run_program(&mut b, 9)?;
    assert_eq!(1001, result);

    let mut c = long.clone();
    result = run_program(&mut c, 8)?;
    assert_eq!(1000, result);

    Ok(())
}

fn main() -> Result<()> {
    let mut program = read_input()?;
    let part1 = run_program(&mut program, 1)?;
    println!("part1: {}", part1);

    program = read_input()?;
    let part2 = run_program(&mut program, 5)?;
    println!("part2: {}", part2);

    Ok(())
}
