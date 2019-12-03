use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ops::Add;

use anyhow::{Context, Result};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    U(usize),
    D(usize),
    L(usize),
    R(usize),
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct Point {
    x: i32,
    y: i32,
}

impl Add for Point {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Point {
    fn new(x: i32, y: i32) -> Point {
        return Point { x, y };
    }
    fn distance(self) -> i32 {
        self.x.abs() + self.y.abs()
    }

    fn step(self, direction: Direction) -> Vec<Point> {
        let mut sub_points = Vec::new();
        let mut location = self;
        let (distance, step) = match direction {
            Direction::U(d) => (d, Point::new(0, -1)),
            Direction::L(d) => (d, Point::new(-1, 0)),
            Direction::R(d) => (d, Point::new(1, 0)),
            Direction::D(d) => (d, Point::new(0, 1)),
        };
        for _ in 0..distance {
            location = location + step;
            sub_points.push(location);
        }
        sub_points
    }
}

fn walk(directions: &Vec<Direction>) -> Vec<Point> {
    let mut location = CENTER;
    let mut path = Vec::new();

    for direction in directions {
        let steps = location.step(*direction);
        for step in steps {
            location = step;
            path.push(step);
        }
    }

    path
}

type Directions = Vec<Vec<Direction>>;

const CENTER: Point = Point { x: 0, y: 0 };

fn parse_line(line: &str) -> Option<Vec<Direction>> {
    line.split(',')
        .map(|e| {
            let n = e
                .chars()
                .skip(1)
                .collect::<String>()
                .parse::<usize>()
                .ok()?;
            match e.chars().next()? {
                'U' => Some(Direction::U(n)),
                'D' => Some(Direction::D(n)),
                'L' => Some(Direction::L(n)),
                'R' => Some(Direction::R(n)),
                _ => None,
            }
        })
        .collect()
}

#[test]
fn test_parse_line() {
    let result = parse_line("R75,D30,L83,U2");
    assert_eq!(
        result,
        Some(vec!(
            Direction::R(75),
            Direction::D(30),
            Direction::L(83),
            Direction::U(2)
        ))
    )
}

fn read_input() -> Result<Directions> {
    let input = File::open("input/day3.txt")?;
    let buffered = BufReader::new(input);

    buffered
        .lines()
        .map(|line| parse_line(line?.as_ref()).context("failed to parse line"))
        .collect()
}

fn part1(all_directions: &Directions) -> Result<i32> {
    let intersections = all_directions
        .iter()
        .map(|directions| walk(directions).into_iter().collect::<HashSet<Point>>())
        .fold(None, |acc: Option<HashSet<Point>>, val| match acc {
            Some(set) => Some(set.intersection(&val).map(|i| *i).collect()),
            None => Some(val),
        })
        .context("directions is empty")?;

    intersections
        .iter()
        .map(|p| p.distance())
        .min()
        .context("no cross points found")
}

#[test]
fn test_part1() -> Result<()> {
    let input_short = vec![
        parse_line("R8,U5,L5,D3").context("parse failed")?,
        parse_line("U7,R6,D4,L4").context("parse failed")?,
    ];

    assert_eq!(6, part1(&input_short)?);

    let input = vec![
        parse_line("R75,D30,R83,U83,L12,D49,R71,U7,L72").context("parse failed")?,
        parse_line("U62,R66,U55,R34,D71,R55,D58,R83").context("parse failed")?,
    ];

    assert_eq!(159, part1(&input)?);

    let input2 = vec![
        parse_line("R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51").context("parse failed")?,
        parse_line("U98,R91,D20,R16,D67,R40,U7,R15,U6,R7").context("parse failed")?,
    ];

    assert_eq!(135, part1(&input2)?);

    Ok(())
}

fn part2(all_directions: &Directions) -> Result<usize> {
    let walks: Vec<Vec<Point>> = all_directions
        .iter()
        .map(|directions| walk(directions))
        .collect();

    let intersections = walks
        .iter()
        .map(|walk| walk.into_iter().map(|i| *i).collect::<HashSet<Point>>())
        .fold(None, |acc: Option<HashSet<Point>>, val| match acc {
            Some(set) => Some(set.intersection(&val).map(|i| *i).collect()),
            None => Some(val),
        })
        .context("directions is empty")?;

    intersections
        .iter()
        .map(|point| {
            // find it in the walks and add the distances.
            let point_sum: usize = walks
                .iter()
                .map(|walk| {
                    let distance = walk.iter().position(|x| x == point)?;
                    // The 0th position is actually 1 distance so we need to add `1` here.
                    Some(distance + 1)
                })
                .collect::<Option<Vec<usize>>>()?
                .iter()
                .sum();

            Some(point_sum)
        })
        .min()
        .context("failed")?
        .context("failed")
}

#[test]
fn test_part2() -> Result<()> {
    let input = vec![
        parse_line("R75,D30,R83,U83,L12,D49,R71,U7,L72").context("parse failed")?,
        parse_line("U62,R66,U55,R34,D71,R55,D58,R83").context("parse failed")?,
    ];

    assert_eq!(610, part2(&input)?);

    let input2 = vec![
        parse_line("R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51").context("parse failed")?,
        parse_line("U98,R91,D20,R16,D67,R40,U7,R15,U6,R7").context("parse failed")?,
    ];

    assert_eq!(410, part2(&input2)?);

    Ok(())
}
fn main() -> Result<()> {
    let input = read_input()?;

    let part1_answer = part1(&input)?;
    println!("part1: {}", part1_answer);

    let part2_answer = part2(&input)?;
    println!("part2: {}", part2_answer);

    Ok(())
}
