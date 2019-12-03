use std::cmp;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

use anyhow::{Context, Result};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    U(i32),
    D(i32),
    L(i32),
    R(i32),
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Point {
        return Point { x, y };
    }
    fn manhatten_distance(self, other: Point) -> i32 {
        (self.x.abs() - other.x.abs()).abs() + (self.y.abs() - other.y.abs()).abs()
    }

    fn points_between(self, other: Point, direction: Direction) -> Vec<Point> {
        let dir_mod = match direction {
            Direction::U(_) | Direction::L(_) => 0,
            Direction::D(_) | Direction::R(_) => 1,
        };
        match direction {
            Direction::U(_) | Direction::D(_) => {
                let min = cmp::min(self.y, other.y);
                let max = cmp::max(self.y, other.y);
                (min + dir_mod..max + dir_mod)
                    .map(|i| Point::new(self.x, i))
                    .collect()
            }
            Direction::L(_) | Direction::R(_) => {
                let min = cmp::min(self.x, other.x);
                let max = cmp::max(self.x, other.x);
                (min + dir_mod..max + dir_mod)
                    .map(|i| Point::new(i, self.y))
                    .collect()
            }
        }
    }

    fn add_direction(mut self, direction: Direction) -> (Point, Vec<Point>) {
        let original = self;
        match direction {
            Direction::U(d) => {
                self.y -= d;
            }
            Direction::D(d) => {
                self.y += d;
            }
            Direction::L(d) => {
                self.x -= d;
            }
            Direction::R(d) => {
                self.x += d;
            }
        }
        (self, self.points_between(original, direction))
    }
}

#[test]
fn test_manhatten_distance() {
    assert_eq!(3, CENTER.manhatten_distance(Point::new(3, 0)));
    assert_eq!(3, CENTER.manhatten_distance(Point::new(-3, 0)));
    assert_eq!(3, CENTER.manhatten_distance(Point::new(0, 3)));
    assert_eq!(3, CENTER.manhatten_distance(Point::new(0, -3)));
    assert_eq!(5, CENTER.manhatten_distance(Point::new(2, 3)));
    assert_eq!(5, CENTER.manhatten_distance(Point::new(2, -3)));
    assert_eq!(5, CENTER.manhatten_distance(Point::new(-2, 3)));
    assert_eq!(5, CENTER.manhatten_distance(Point::new(-2, -3)));
}
#[test]
fn test_points_between() {
    let a = Point { x: 0, y: 0 };

    let points = a.points_between(Point::new(0, 2), Direction::D(2));
    assert_eq!(points, vec!(Point::new(0, 1), Point::new(0, 2)));

    let other_points = a.points_between(Point::new(-1, 0), Direction::L(1));
    assert_eq!(other_points, vec!(Point::new(-1, 0)))
}

#[test]
fn test_add_direction() {
    let (new_location, points) = CENTER.add_direction(Direction::D(2));
    assert_eq!(new_location, Point::new(0, 2));
    assert_eq!(points, vec!(Point::new(0, 1), Point::new(0, 2)));

    let (new_location2, points2) = CENTER.add_direction(Direction::U(2));
    assert_eq!(new_location2, Point::new(0, -2));
    assert_eq!(points2, vec!(Point::new(0, -2), Point::new(0, -1)));

    let (new_location3, points3) = CENTER.add_direction(Direction::R(2));
    assert_eq!(new_location3, Point::new(2, 0));
    assert_eq!(points3, vec!(Point::new(1, 0), Point::new(2, 0)));

    let (new_location4, points4) = CENTER.add_direction(Direction::L(2));
    assert_eq!(new_location4, Point::new(-2, 0));
    assert_eq!(points4, vec!(Point::new(-2, 0), Point::new(-1, 0)));

    let (new_location5, points5) = Point::new(1, 0).add_direction(Direction::L(2));
    assert_eq!(new_location5, Point::new(-1, 0));
    assert_eq!(points5, vec!(Point::new(-1, 0), Point::new(0, 0)));

    let (new_location6, points6) = Point::new(-1, 0).add_direction(Direction::R(2));
    assert_eq!(new_location6, Point::new(1, 0));
    assert_eq!(points6, vec!(Point::new(0, 0), Point::new(1, 0)));
}

type Directions = Vec<Vec<Direction>>;
type Grid = HashMap<Point, HashSet<usize>>;

const CENTER: Point = Point { x: 0, y: 0 };

fn walk(all_directions: &Directions) -> Grid {
    let mut grid = HashMap::new();
    let mut wire_index = 0;

    for directions in all_directions {
        let mut location = CENTER;
        for direction in directions {
            let (new_point, marked_points) = location.add_direction(*direction);
            location = new_point;
            for point in marked_points {
                let wires = grid.entry(point).or_insert(HashSet::new());
                wires.insert(wire_index);
            }
        }
        wire_index += 1;
    }
    grid
}

fn parse_line(line: &str) -> Option<Vec<Direction>> {
    line.split(',')
        .map(|e| {
            let n = e.chars().skip(1).collect::<String>().parse::<i32>().ok()?;
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
    let grid = walk(all_directions);

    grid.iter()
        .filter(|(_, v)| v.len() > 1)
        .map(|(k, _)| k.manhatten_distance(CENTER))
        .min()
        .context("failed to find any cross points")
}

fn walk_to_cross_point(directions: &Vec<Direction>, destination: Point) -> usize {
    let mut distance = 0;
    let mut location = CENTER;
    for direction in directions {
        let (new_point, marked_points) = location.add_direction(*direction);
        if let Some(index) = marked_points.iter().position(|p| *p == destination) {
            // TODO: may not work since depends on walk direction.
            println!("{} {}", marked_points.len(), index);

            match direction {
                Direction::U(_) | Direction::L(_) => return distance + marked_points.len() - index,
                Direction::D(_) | Direction::R(_) => return distance + index + 1,
            };
        } else {
            location = new_point;
            distance += marked_points.len();
        }
    }

    //
    0
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
    let grid = walk(all_directions);

    let cross_points: Vec<Point> = grid
        .iter()
        .filter(|(_, v)| v.len() > 1)
        .map(|(k, _)| *k)
        .collect();

    cross_points
        .iter()
        .map(|destination| {
            println!("");
            all_directions
                .iter()
                .map(|directions| walk_to_cross_point(&directions, *destination))
                .collect()
        })
        .map(|distances: Vec<usize>| distances.iter().sum())
        .min()
        .context("No cross points found")
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
