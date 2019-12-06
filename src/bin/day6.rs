use std::collections::HashMap;
use std::fs;

use anyhow::{Context, Result};
#[derive(Debug, PartialEq, Eq)]
enum Orbits {
    NoMoons(String),
    HasMoons(String, Vec<Orbits>),
}

fn walk(orbits: &Orbits) -> usize {
    sub_walk(orbits, 0)
}
fn sub_walk(orbits: &Orbits, parents: usize) -> usize {
    match orbits {
        Orbits::NoMoons(_) => parents - 1,
        Orbits::HasMoons(_, children) => {
            let child_sum: usize = children.iter().map(|c| sub_walk(c, parents + 1)).sum();
            let indirect = if parents >= 1 { parents - 1 } else { 0 };
            indirect + children.len() + child_sum
        }
    }
}

#[test]
fn test_walk() {
    let mut tree = Orbits::HasMoons("COM".to_owned(), vec![Orbits::NoMoons("A".to_owned())]);

    assert_eq!(1, walk(&tree));

    tree = Orbits::HasMoons(
        "COM".to_owned(),
        vec![Orbits::HasMoons(
            "A".to_owned(),
            vec![Orbits::NoMoons("B".to_owned())],
        )],
    );

    assert_eq!(3, walk(&tree));

    tree = Orbits::HasMoons(
        "COM".to_owned(),
        vec![Orbits::HasMoons(
            "A".to_owned(),
            vec![Orbits::HasMoons(
                "B".to_owned(),
                vec![Orbits::NoMoons("C".to_owned())],
            )],
        )],
    );

    assert_eq!(6, walk(&tree));

    tree = Orbits::HasMoons(
        "COM".to_owned(),
        vec![Orbits::HasMoons(
            "A".to_owned(),
            vec![
                Orbits::NoMoons("B".to_owned()),
                Orbits::HasMoons("C".to_owned(), vec![Orbits::NoMoons("D".to_owned())]),
            ],
        )],
    );

    assert_eq!(8, walk(&tree));
}

fn convert(us: &str, edges: &HashMap<String, Vec<String>>) -> Orbits {
    match edges.get(us) {
        Some(children) => Orbits::HasMoons(
            us.to_owned(),
            children.iter().map(|c| convert(c, edges)).collect(),
        ),
        None => Orbits::NoMoons(us.to_owned()),
    }
}

fn read_input() -> Result<Orbits> {
    let contents = fs::read_to_string("input/day6.txt")?;
    parse(&contents)
}

fn parse(input: &str) -> Result<Orbits> {
    let hash: HashMap<String, Vec<String>> = input
        .lines()
        .map(|line| {
            let split: Vec<String> = line.trim().split(')').map(|s| s.to_owned()).collect();
            Ok((
                split.first().context("invalid input")?.to_owned(),
                split.last().context("invalid input")?.to_owned(),
            ))
        })
        .fold(
            HashMap::new(),
            |mut acc: HashMap<String, Vec<String>>, maybe_orbit: Result<(String, String)>| {
                match maybe_orbit {
                    Ok((a, b)) => {
                        match acc.get_mut(&a) {
                            Some(v) => {
                                v.push(b);
                            }
                            None => {
                                // b orbits a so COM will be a key.
                                acc.insert(a, vec![b]);
                            }
                        }
                        acc
                    }
                    Err(_) => acc,
                }
            },
        );

    Ok(convert("COM", &hash))
}

#[test]
fn test_all() -> Result<()> {
    let input = "COM)B
                 B)C
                 C)D
                 D)E
                 E)F
                 B)G
                 G)H
                 D)I
                 E)J
                 J)K
                 K)L";
    /*
            G - H       J - K - L
           /           /
    COM - B - C - D - E - F
                   \
                    I
    */
    let parsed = parse(input)?;
    assert_eq!(42, walk(&parsed));

    Ok(())
}

fn find(orbits: &Orbits, to_find: &str) -> Option<Vec<String>> {
    match orbits {
        Orbits::NoMoons(name) => {
            if name == to_find {
                Some(vec![name.to_owned()])
            } else {
                None
            }
        }
        Orbits::HasMoons(name, children) => {
            // for any of these if they are not empty then build them up
            let mut found_paths: Vec<String> = children
                .iter()
                .map(|c| find(c, to_find))
                .find(|p| p.is_some())??;
            found_paths.push(name.to_owned());
            Some(found_paths)
        }
    }
}

#[test]
fn test_find() -> Result<()> {
    let tree = Orbits::HasMoons(
        "COM".to_owned(),
        vec![Orbits::HasMoons(
            "A".to_owned(),
            vec![Orbits::NoMoons("B".to_owned())],
        )],
    );

    assert_eq!(
        vec!("B", "A", "COM"),
        find(&tree, "B").context("failed to find")?
    );
    Ok(())
}

fn part2(orbits: &Orbits) -> Result<usize> {
    let mut you = find(orbits, "YOU").context("failed to find YOU")?;
    let mut san = find(orbits, "SAN").context("failed to find SAN")?;

    you.reverse();
    san.reverse();

    let common_ancestor_position = you
        .iter()
        .zip(san.iter())
        .position(|(a, b)| {
            // One before this is the common ancestor.
            a != b
        })
        .context("failed to find common ancestor")?;

    let you_distance_to_common = you.len() - common_ancestor_position - 1;
    let san_distance_to_common = san.len() - common_ancestor_position - 1;

    Ok(you_distance_to_common + san_distance_to_common)
}

#[test]
fn test_part2() -> Result<()> {
    let input = "COM)B
                 B)C
                 C)D
                 D)E
                 E)F
                 B)G
                 G)H
                 D)I
                 E)J
                 J)K
                 K)L
                 K)YOU
                 I)SAN";

    let parsed = parse(input)?;
    assert_eq!(4, part2(&parsed)?);

    Ok(())
}
fn main() -> Result<()> {
    let input = read_input()?;
    let part1 = walk(&input);
    println!("part1: {}", part1);

    let part2 = part2(&input)?;
    println!("part2: {}", part2);
    Ok(())
}
