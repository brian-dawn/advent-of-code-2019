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
}

type Memory = Vec<i64>;

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
            _ => Err(anyhow!("unknown mode {}", mode)),
        }
    }
    fn realize(self, memory: &Memory) -> Result<i64> {
        match self {
            Parameter::PositionMode(n) => {
                let u: usize = n.try_into()?;
                let out: &i64 = memory
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
        .map(|s| Ok(s.trim().parse::<i64>()?))
        .collect()
}

struct CPU {
    mem: Memory,
    pc: usize,
    last_output: Option<i64>,
    inputs: Vec<i64>,
}

enum Status<T> {
    Ready(T),
    Halted(T),
}

impl CPU {
    fn new(memory: &Memory) -> CPU {
        CPU {
            mem: memory.clone(),
            pc: 0,
            last_output: None,
            inputs: Vec::new(),
        }
    }

    fn add_input(&mut self, input: i64) {
        self.inputs.push(input)
    }
    fn step(&mut self) -> Result<Status<i64>> {
        println!("{:?}", self.inputs);
        loop {
            let code = self.mem.get(self.pc).context("read failed")?;
            let modes: OpCodeMode = (*code).into();

            let raw1 = self.mem.get(self.pc + 1).map(|i| *i);
            let raw2 = self.mem.get(self.pc + 2).map(|i| *i);
            let raw3 = self.mem.get(self.pc + 3).map(|i| *i);

            let p1 = raw1.context("invalid program p1");
            let _p2 = raw2.context("invalid program p2");
            let p3 = raw3.context("invalid program p3");

            let real_p1 = raw1
                .context("invalid program realp1")
                .and_then(|i| Parameter::build(modes.p1, i)?.realize(&self.mem));
            let real_p2 = raw2
                .context("invalid program realp2")
                .and_then(|i| Parameter::build(modes.p2, i)?.realize(&self.mem));
            let _real_p3 = raw3
                .context("invalid program realp3")
                .and_then(|i| Parameter::build(modes.p3, i)?.realize(&self.mem));

            match modes.opcode {
                1 => {
                    self.pc += 4;
                    let output_addr: usize = p3?.try_into()?;
                    self.mem[output_addr] = real_p1? + real_p2?;
                }

                2 => {
                    self.pc += 4;
                    let output_addr: usize = p3?.try_into()?;
                    self.mem[output_addr] = real_p1? * real_p2?;
                }

                3 => {
                    self.pc += 2;
                    let addr: usize = p1?.try_into()?;
                    let inp = self.inputs.pop().context("ran out of inputs")?;
                    self.mem[addr] = inp;
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
                    self.mem[output_addr] = if real_p1? < real_p2? { 1 } else { 0 };
                }

                8 => {
                    self.pc += 4;

                    let output_addr: usize = p3?.try_into()?;
                    self.mem[output_addr] = if real_p1? == real_p2? { 1 } else { 0 };
                }
                99 => break,
                op => {
                    return Err(anyhow!("unknown opcode {}", modes.opcode));
                }
            };
        }

        return Ok(Status::Halted(self.last_output.context("No output")?));
    }
}

fn process_phase(program: &Memory, phase: &[i64]) -> Result<i64> {
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

fn find_biggest_phase(program: &Memory) -> Result<i64> {
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

/// Helper function for running a oneshot program on a CPU.
fn run_program(memory: &Memory, input: &Vec<i64>) -> Result<i64> {
    let mut cpu = CPU::new(memory);
    cpu.inputs = input.clone();
    match cpu.step()? {
        Status::Ready(out) => Ok(out),
        Status::Halted(out) => Ok(out),
    }
}

fn part2(program: &Memory) -> Result<i64> {
    let mut data = [0, 1, 2, 3, 4];
    let mut heap = Heap::new(&mut data);

    let mut max_power = 0;
    while let Some(phase) = heap.next_permutation() {
        let power = process_phase(&program, phase)?;

        // Create 5 CPUs with the phase input as the first input.
        let mut cpus: Vec<CPU> = phase
            .iter()
            .map(|phase| {
                let mut cpu = CPU::new(program);
                cpu.inputs = vec![*phase];
                cpu
            })
            .collect();

        // Start at 0.
        cpus[0].add_input(0);

        let mut power = 0;
        let mut index = 0;
        loop {
            match cpus[index].step()? {
                Status::Ready(output) => {
                    power = output;
                }
                Status::Halted(output) => {
                    power = output;
                    if index == 4 {
                        break;
                    }
                }
            }
            index = (index + 1) % 5;
            cpus[index].add_input(power);
        }

        max_power = std::cmp::max(max_power, power);
    }

    Ok(max_power)
}

fn main() -> Result<()> {
    let program = read_input()?;
    let part1 = find_biggest_phase(&program)?;
    println!("part1: {}", part1);

    let part2 = part2(&program)?;
    println!("part2: {}", part2);
    Ok(())
}

#[test]
fn test_part2() -> Result<()> {
    let mut program = vec![
        3, 26, 1001, 26, -4, 26, 3, 27, 1002, 27, 2, 27, 1, 27, 26, 27, 4, 27, 1001, 28, -1, 28,
        1005, 28, 6, 99, 0, 0, 5,
    ];

    assert_eq!(139629729, part2(&program)?);

    program = vec![
        3, 52, 1001, 52, -5, 52, 3, 53, 1, 52, 56, 54, 1007, 54, 5, 55, 1005, 55, 26, 1001, 54, -5,
        54, 1105, 1, 12, 1, 53, 54, 53, 1008, 54, 0, 55, 1001, 55, 1, 55, 2, 53, 55, 53, 4, 53,
        1001, 56, -1, 56, 1005, 56, 6, 99, 0, 0, 0, 0, 10,
    ];

    assert_eq!(18216, part2(&program)?);

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
