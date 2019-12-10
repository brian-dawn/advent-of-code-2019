#[macro_use]
extern crate anyhow;

use anyhow::{Context, Result};
use std::collections::HashSet;

use std::fs;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Point {
    x: usize,
    y: usize,
}

#[derive(Debug, PartialEq, Eq)]
struct Space {
    width: usize,
    height: usize,
    asteroids: HashSet<Point>,
}

impl Point {
    fn new(x: usize, y: usize) -> Point {
        Point { x, y }
    }
}

fn parse(data: &str) -> Result<Space> {
    let mut out = HashSet::new();
    let height = data.lines().count();
    let mut width = 0;

    for (row_number, line) in data.lines().enumerate() {
        for (col_number, c) in line.trim().chars().enumerate() {
            width = std::cmp::max(col_number, width);
            match c {
                '#' => {
                    out.insert(Point {
                        x: col_number,
                        y: row_number,
                    });
                }
                '.' => {}
                _ => return Err(anyhow!("unknown char {}", c)),
            }
        }
    }

    Ok(Space {
        width: width + 1,
        height,
        asteroids: out,
    })
}

fn angle(p1: Point, p2: Point) -> f64 {
    let y = p2.y as f64 - p1.y as f64;
    let x = p2.x as f64 - p1.x as f64;
    let angle = y.atan2(x) * 180f64 / std::f64::consts::PI;

    let dec = 1_000_000_000_f64;
    (angle * dec).round() / dec
}

fn visible_points(our_point: Point, space: &Space) -> usize {
    space
        .asteroids
        .iter()
        .filter(|point| **point != our_point)
        .map(move |point| {
            let a = angle(our_point, *point);
            (a * 1_000_000_000_f64) as i64
        })
        .collect::<HashSet<i64>>()
        .len()
}

fn part1(input: &str) -> Result<usize> {
    let parsed = parse(input)?;

    let best_count = parsed
        .asteroids
        .iter()
        .map(|our_point| {
            let visible_points = visible_points(*our_point, &parsed);
            (our_point, visible_points)
        })
        .fold(None, |best, (point, count)| {
            match best {
                Some((_, best_count)) => {
                    //
                    if best_count < count {
                        Some((point, count))
                    } else {
                        best
                    }
                }
                None => Some((point, count)),
            }
        })
        .context("no valid points")?
        .1;

    Ok(best_count)
}

fn main() -> Result<()> {
    let contents = fs::read_to_string("input/day10.txt")?;

    let p1 = part1(&contents)?;
    println!("{}", p1);

    Ok(())
}

#[test]
fn test_part1() -> Result<()> {
    let mut example = ".#..#
                         .....
                         #####
                         ....#
                         ...##";

    assert_eq!(8, part1(example)?);

    Ok(())
}

#[test]
fn test_visible_points() -> Result<()> {
    let mut example = ".#..#
                         .....
                         #####
                         ....#
                         ...##";

    let space = parse(example)?;
    assert_eq!(7, visible_points(Point::new(1, 0), &space));
    assert_eq!(5, visible_points(Point::new(4, 2), &space));

    Ok(())
}
#[test]
fn test_angle() {
    assert_eq!(0f64, angle(Point::new(2, 2), Point::new(3, 2)));
    assert_eq!(45f64, angle(Point::new(2, 2), Point::new(3, 3)));
    assert_eq!(45f64, angle(Point::new(2, 2), Point::new(4, 4)));
    assert_eq!(90f64, angle(Point::new(2, 2), Point::new(2, 3)));
    assert_eq!(-45f64, angle(Point::new(2, 2), Point::new(3, 1)));
    assert_eq!(-135f64, angle(Point::new(2, 2), Point::new(1, 1)));
}

#[test]
fn test_parse() -> Result<()> {
    let example = ".#..#
                         .....
                         #####
                         ....#
                         ...##";
    let space = parse(example)?;

    assert_eq!(5, space.width);
    assert_eq!(5, space.height);

    let result = space.asteroids;
    assert!(result.contains(&Point { x: 1, y: 0 }));
    assert!(result.contains(&Point { x: 0, y: 2 }));
    assert!(!result.contains(&Point { x: 0, y: 0 }));

    Ok(())
}
