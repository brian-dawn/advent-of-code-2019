#[macro_use]
extern crate anyhow;

use std::convert::TryInto;
use std::fs;

use anyhow::{Context, Result};

use permutohedron::Heap;

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
    let contents = fs::read_to_string("input/day7.txt")?;
    contents
        .split(',')
        .map(|s| Ok(s.trim().parse::<i32>()?))
        .collect()
}

fn run_program(program: &mut Memory, input: &Vec<i32>) -> Result<i32> {
    let mut index = 0;
    let mut outputs = Vec::new();
    let mut inputs = input.clone();
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
                let inp = inputs.pop().context("ran out of inputs")?;
                program[addr] = inp;
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
            op => {
                return Err(anyhow!("unknown opcode {}", modes.opcode));
            }
        };
    }

    outputs.last().context("no output").map(|i| *i)
}

fn process_phase(program: &Memory, phase: &[i32]) -> Result<i32> {
    let mut prog = program.clone();
    (0..5).fold(Ok(0), |input_signal, index| match input_signal {
        Ok(signal) => {
            let input = vec![signal, phase[index]];
            let output = run_program(&mut prog, &input)?;
            Ok(output)
        }
        Err(_) => input_signal,
    })
}

fn find_biggest_phase(program: &Memory) -> Result<i32> {
    let mut data = [0, 1, 2, 3, 4];
    let mut heap = Heap::new(&mut data);

    let mut max_power = None;
    while let Some(phase) = heap.next_permutation() {
        let power = process_phase(&program, phase)?;

        if let Some(max) = max_power {
            if max < power {
                max_power = Some(power);
            }
        } else {
            max_power = Some(power);
        }
    }
    max_power.context("failed to get max power")
}
fn main() -> Result<()> {
    let program = read_input()?;
    let part1 = find_biggest_phase(&program)?;
    println!("part1: {}", part1);

    Ok(())
}

#[test]
fn test_process_phase() -> Result<()> {
    let mut program = vec![
        3, 23, 3, 24, 1002, 24, 10, 24, 1002, 23, -1, 23, 101, 5, 23, 23, 1, 24, 23, 23, 4, 23, 99,
        0, 0,
    ];

    let mut phase = [0, 1, 2, 3, 4];
    assert_eq!(54321, process_phase(&program, &phase)?);

    program = vec![
        3, 15, 3, 16, 1002, 16, 10, 16, 1, 16, 15, 15, 4, 15, 99, 0, 0,
    ];
    phase = [4, 3, 2, 1, 0];
    assert_eq!(43210, process_phase(&program, &phase)?);

    Ok(())
}

#[test]
fn test_find_biggest_phase() -> Result<()> {
    //
    let mut program = vec![
        3, 31, 3, 32, 1002, 32, 10, 32, 1001, 31, -2, 31, 1007, 31, 0, 33, 1002, 33, 7, 33, 1, 33,
        31, 31, 1, 32, 31, 31, 4, 31, 99, 0, 0, 0,
    ];
    assert_eq!(65210, find_biggest_phase(&program)?);

    program = vec![
        3, 23, 3, 24, 1002, 24, 10, 24, 1002, 23, -1, 23, 101, 5, 23, 23, 1, 24, 23, 23, 4, 23, 99,
        0, 0,
    ];
    assert_eq!(54321, find_biggest_phase(&program)?);

    program = vec![
        3, 15, 3, 16, 1002, 16, 10, 16, 1, 16, 15, 15, 4, 15, 99, 0, 0,
    ];
    assert_eq!(43210, find_biggest_phase(&program)?);

    Ok(())
}

#[test]
fn test_run_program() -> Result<()> {
    // Program test input == 8
    let mut program = vec![3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8];
    let mut result = run_program(&mut program, &vec![8])?;
    assert_eq!(1, result);

    program = vec![3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8];
    result = run_program(&mut program, &vec![3])?;
    assert_eq!(0, result);

    // Program test input < 8
    program = vec![3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8];
    result = run_program(&mut program, &vec![4])?;
    assert_eq!(1, result);

    program = vec![3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8];
    result = run_program(&mut program, &vec![99])?;
    assert_eq!(0, result);

    // Program test input == 8 immediate mode
    program = vec![3, 3, 1108, -1, 8, 3, 4, 3, 99];
    result = run_program(&mut program, &vec![8])?;
    assert_eq!(1, result);

    program = vec![3, 3, 1108, -1, 8, 3, 4, 3, 99];
    result = run_program(&mut program, &vec![10])?;
    assert_eq!(0, result);

    // Program test input < 8 immediate mode
    program = vec![3, 3, 1107, -1, 8, 3, 4, 3, 99];
    result = run_program(&mut program, &vec![4])?;
    assert_eq!(1, result);

    program = vec![3, 3, 1107, -1, 8, 3, 4, 3, 99];
    result = run_program(&mut program, &vec![99])?;
    assert_eq!(0, result);

    // Jump tests for zero.
    program = vec![3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, -1, 0, 1, 9];
    result = run_program(&mut program, &vec![0])?;
    assert_eq!(0, result);

    program = vec![3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, -1, 0, 1, 9];
    result = run_program(&mut program, &vec![-1])?;
    assert_eq!(1, result);

    // Jump tests for zero immediate mode
    program = vec![3, 3, 1105, -1, 9, 1101, 0, 0, 12, 4, 12, 99, 1];
    result = run_program(&mut program, &vec![0])?;
    assert_eq!(0, result);

    program = vec![3, 3, 1105, -1, 9, 1101, 0, 0, 12, 4, 12, 99, 1];
    result = run_program(&mut program, &vec![-1])?;
    assert_eq!(1, result);

    // Long example
    let long = vec![
        3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31, 1106, 0, 36, 98, 0, 0,
        1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104, 999, 1105, 1, 46, 1101, 1000, 1, 20, 4, 20,
        1105, 1, 46, 98, 99,
    ];

    let mut a = long.clone();
    result = run_program(&mut a, &vec![7])?;
    assert_eq!(999, result);

    let mut b = long.clone();
    result = run_program(&mut b, &vec![9])?;
    assert_eq!(1001, result);

    let mut c = long.clone();
    result = run_program(&mut c, &vec![8])?;
    assert_eq!(1000, result);

    Ok(())
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
