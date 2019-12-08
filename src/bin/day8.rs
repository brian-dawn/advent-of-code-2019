use std::convert::TryInto;
use std::fs;

use anyhow::{Context, Result};

#[derive(Debug, PartialEq, Eq)]
struct Image {
    width: usize,
    height: usize,
    layers: usize,
    data: Vec<u8>, // data is stored contiguously for performance.
}

impl Image {
    fn new(width: usize, height: usize, layers: usize, data: Vec<u8>) -> Image {
        Image {
            width,
            height,
            layers,
            data,
        }
    }

    fn decode(input: &str, width: usize, height: usize) -> Result<Image> {
        let maybe_data: Result<Vec<u8>> = input
            .chars()
            .map(|c| {
                let u: u8 = c
                    .to_digit(10)
                    .context("failed to convert char to digit")?
                    .try_into()?;
                Ok(u)
            })
            .collect();
        let data = maybe_data?;

        let layers = data.len() / width / height;

        Ok(Image::new(width, height, layers, data))
    }

    /// Return all the pixels in each layer.
    fn layers<'a>(&'a self) -> Vec<&'a [u8]> {
        let layers: Vec<&[u8]> = self.data.chunks(self.width * self.height).collect();
        layers
    }
}

fn part1(input: &str) -> Result<i32> {
    let image = Image::decode(input, 25, 6)?;
    let layers = image.layers();

    let (best_index, _) = layers
        .iter()
        .enumerate()
        .map(|(index, layer)| (index, layer.iter().filter(|e| **e == 0u8).count()))
        .fold(None, |acc, (index, num_zeroes)| match acc {
            Some((_, best_num_zeroes)) => {
                if best_num_zeroes > num_zeroes {
                    Some((index, num_zeroes))
                } else {
                    acc
                }
            }
            None => Some((index, num_zeroes)),
        })
        .context("empty image")?;

    let best_layer = layers.get(best_index).context("failed to get best layer")?;
    let (num1, num2) = best_layer
        .iter()
        .fold((0, 0), |(num1, num2), val| match val {
            1 => (num1 + 1, num2),
            2 => (num1, num2 + 1),
            _ => (num1, num2),
        });

    Ok(num1 * num2)
}

fn main() -> Result<()> {
    let input = fs::read_to_string("input/day8.txt")?;
    let input = input.trim();

    let part1 = part1(&input)?;
    println!("part1: {}", part1);

    Ok(())
}

#[test]
fn test_layers() -> Result<()> {
    let image = Image::decode("123456789012", 3, 2)?;
    let layers = image.layers();

    let a = [1, 2, 3, 4, 5, 6];
    let b = [7, 8, 9, 0, 1, 2];
    assert_eq!(vec![&a, &b], layers);

    Ok(())
}

#[test]
fn test_decode() -> Result<()> {
    let image = Image::decode("123456789012", 3, 2)?;
    assert_eq!(3, image.width);
    assert_eq!(2, image.height);
    assert_eq!(2, image.layers);

    Ok(())
}
