use std::fs::File;
use std::io::{BufRead, BufReader};
use std::iter::Sum;

fn read_input() -> Option<Vec<i32>> {
    let input = File::open("input/day1.txt").ok()?;
    let buffered = BufReader::new(input);

    buffered
        .lines()
        .map(|line| Some(line.ok()?.parse::<i32>().ok()?))
        .collect()
}

fn compute_fuel(mass: i32) -> i32 {
    mass / 3 - 2
}

#[test]
fn test_compute_fuel() {
    assert_eq!(654, compute_fuel(1969));
    assert_eq!(33583, compute_fuel(100756));
}

fn compute_fuel_fuel(mass: i32) -> i32 {
    let cost = compute_fuel(mass);
    if cost < 0 {
        return 0;
    }
    return cost + compute_fuel_fuel(cost);
}

#[test]
fn test_compute_fuel_fuel() {
    assert_eq!(50346, compute_fuel_fuel(100756));
}

fn main() {
    let input = read_input().expect("failed to load input");
    let part1: i32 = input.iter().map(|i| compute_fuel(*i)).sum();

    println!("part1: {}", part1);

    let part2: i32 = input.iter().map(|i| compute_fuel_fuel(*i)).sum();

    println!("part2: {}", part2);
}
