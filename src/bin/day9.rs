#[macro_use]
extern crate anyhow;

use std::convert::TryInto;
use std::fs;

use anyhow::{Context, Result};

use permutohedron::Heap;

#[derive(Debug, PartialEq, Eq)]
enum Parameter {
    PositionMode(i64),
    ImmediateMode(i64),
    RelativeMode(i64),
}

#[derive(Debug, PartialEq, Eq)]
struct OpCodeMode {
    opcode: u8,
    p1: u8,
    p2: u8,
    p3: u8,
}

impl From<i64> for OpCodeMode {
    fn from(code: i64) -> OpCodeMode {
        OpCodeMode {
            opcode: (code % 100) as u8,
            p1: ((code / 100) % 10) as u8,
            p2: ((code / 1000) % 10) as u8,
            p3: ((code / 10000) % 10) as u8,
        }
    }
}

impl Parameter {
    fn build(mode: u8, parameter: i64) -> Result<Parameter> {
        match mode {
            0 => Ok(Parameter::PositionMode(parameter)),
            1 => Ok(Parameter::ImmediateMode(parameter)),
            2 => Ok(Parameter::RelativeMode(parameter)),
            _ => Err(anyhow!("unknown mode {}", mode)),
        }
    }
    fn realize(self, memory: &[i64], relative_base: i64) -> Result<i64> {
        match self {
            Parameter::PositionMode(n) => {
                let u: usize = n.try_into()?;
                let out: &i64 = memory
                    .get(u)
                    .unwrap_or(&0);
                Ok(*out)
            }
            Parameter::ImmediateMode(n) => Ok(n),
            Parameter::RelativeMode(n) => {
                let u: usize = (relative_base + n).try_into()?;
                let out: &i64 = memory
                    .get(u)
                    .unwrap_or(&0);
                Ok(*out)
            }
        }
    }
}

fn read_input() -> Result<Vec<i64>> {
    let contents = fs::read_to_string("input/day9.txt")?;
    contents
        .split(',')
        .map(|s| Ok(s.trim().parse::<i64>()?))
        .collect()
}

struct CPU {
    mem: Vec<i64>,
    pc: usize,
    last_output: Option<i64>,
    inputs: Vec<i64>,
    relative_base: i64,
}

#[derive(Debug, PartialEq, Eq)]
enum Status<T> {
    Ready(T),
    Halted(T),
}

impl CPU {
    fn new(memory: &[i64]) -> CPU {
        CPU {
            mem: memory.to_vec(),
            pc: 0,
            last_output: None,
            inputs: Vec::new(),
            relative_base: 0,
        }
    }

    fn add_input(&mut self, input: i64) {
        self.inputs.push(input)
    }

    fn set_memory(&mut self, position: usize, value: i64) {

        for _ in self.mem.len()..position {
            self.mem.push(0)
        }
    }

    fn step(&mut self) -> Result<Status<i64>> {
        loop {
            let code = self.mem.get(self.pc).context("read failed")?;
            let modes: OpCodeMode = (*code).into();

            let raw1 = self.mem.get(self.pc + 1).copied();
            let raw2 = self.mem.get(self.pc + 2).copied();
            let raw3 = self.mem.get(self.pc + 3).copied();

            let p1 = raw1.context("invalid program p1");
            let _p2 = raw2.context("invalid program p2");
            let p3 = raw3.context("invalid program p3");

            let real_p1 = raw1.context("invalid program realp1").and_then(|i| {
                Parameter::build(modes.p1, i)?.realize(&self.mem, self.relative_base)
            });
            let real_p2 = raw2.context("invalid program realp2").and_then(|i| {
                Parameter::build(modes.p2, i)?.realize(&self.mem, self.relative_base)
            });
            let _real_p3 = raw3.context("invalid program realp3").and_then(|i| {
                Parameter::build(modes.p3, i)?.realize(&self.mem, self.relative_base)
            });


            match modes.opcode {
                1 => {
                    self.pc += 4;
                    let output_addr: usize = p3?.try_into()?;
                    self.set_memory(output_addr, real_p1? + real_p2?);
                }

                2 => {
                    self.pc += 4;
                    let output_addr: usize = p3?.try_into()?;
                    self.set_memory(output_addr, real_p1? * real_p2?);
                }

                3 => {
                    self.pc += 2;
                    let addr: usize = p1?.try_into()?;
                    let inp: &i64 = self.inputs.first().context("ran out of inputs")?;
                    self.set_memory(addr, *inp);
                    let rest = self.inputs.iter().skip(1).copied().collect::<Vec<i64>>();
                    self.inputs = rest;
                }

                4 => {
                    self.pc += 2;
                    let val = real_p1?;
                    self.last_output = Some(val);
                    return Ok(Status::Ready(val));
                }
                5 => {
                    self.pc += 3;
                    if real_p1? != 0 {
                        self.pc = real_p2?.try_into()?;
                    }
                }
                6 => {
                    self.pc += 3;
                    if real_p1? == 0 {
                        self.pc = real_p2?.try_into()?;
                    }
                }
                7 => {
                    self.pc += 4;

                    let output_addr: usize = p3?.try_into()?;
                    self.set_memory(output_addr, if real_p1? < real_p2? { 1 } else { 0 });
                }

                8 => {
                    self.pc += 4;

                    let output_addr: usize = p3?.try_into()?;
                    self.set_memory(output_addr, if real_p1? == real_p2? { 1 } else { 0 });
                }
                9 => {
                    self.pc += 2;
                    self.relative_base += real_p1?;
                }
                99 => break,
                _ => {
                    return Err(anyhow!("unknown opcode {}", modes.opcode));
                }
            };
        }

        Ok(Status::Halted(self.last_output.context("No output")?))
    }
}

/// Helper function for running a oneshot program on a CPU.
fn run_program(memory: &[i64], input: &[i64]) -> Result<i64> {
    let mut cpu = CPU::new(memory);
    cpu.inputs = input.to_vec();
    match cpu.step()? {
        Status::Ready(out) => Ok(out),
        Status::Halted(out) => Ok(out),
    }
}

fn main() -> Result<()> {
    let program = read_input()?;
    let mut cpu = CPU::new(&program);
    cpu.inputs = vec![1];
    let part1 = match cpu.step()? {
        Status::Ready(out) => out,
        Status::Halted(out) => out
    };
    println!("part1: {}", part1);

    Ok(())
}

#[test]
fn test_part1() -> Result<()> {
    let program = vec![
        109, 1, 204, -1, 1001, 100, 1, 100, 1008, 100, 16, 101, 1006, 101, 0, 99,
    ];
    let mut cpu = CPU::new(&program);

    let mut outs = Vec::new();
    loop {
        match cpu.step()? {
            Status::Ready(val) => outs.push(val),
            Status::Halted(_) => break,
        }
    }
    assert_eq!(program, outs);

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
