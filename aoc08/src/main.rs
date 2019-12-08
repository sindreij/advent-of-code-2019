use std::io::{self, Read};

use anyhow::Result;
use itertools::Itertools;

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    println!("Part1: {}", part1(&input)?);
    println!("Part2: {}", part2(&input)?);

    Ok(())
}

fn part1(input: &str) -> Result<usize> {
    let min_layer = parse_image(input, 25, 6)?
        .into_iter()
        .min_by_key(|layer| layer.iter().filter(|digit| **digit == 0).count())
        .unwrap();

    let num_ones = min_layer.iter().filter(|digit| **digit == 1).count();
    let num_twos = min_layer.iter().filter(|digit| **digit == 2).count();

    Ok(num_ones * num_twos)
}

fn part2(input: &str) -> Result<i32> {
    println!("Hello World");
    let (width, height) = (25, 6);
    let image = collapse_layers(parse_image(input, width, height)?);

    for (i, pixel) in image.into_iter().enumerate() {
        if i % width == 0 {
            println!("");
        }
        print!("{}", pixel);
    }

    println!("");

    Ok(-1)
}

fn parse_image(data: &str, width: usize, height: usize) -> Result<Vec<Vec<i32>>> {
    let data = data
        .chars()
        .filter_map(|a| a.to_string().parse().ok())
        .collect::<Vec<i32>>();
    assert_eq!(dbg!(data.len()) % (width * height), 0);

    Ok(data
        .into_iter()
        .batching(|it| {
            let result = it.take(width * height).collect::<Vec<i32>>();
            if result.is_empty() {
                None
            } else {
                Some(result)
            }
        })
        .collect())
}

fn collapse_layers(image: Vec<Vec<i32>>) -> Vec<i32> {
    // image is a list of layers where each layer is pixels for that layer
    // pixels is a list of pixel where each pixel is a list of all layers

    let mut pixels: Vec<Vec<i32>> = Vec::new();
    for layer in image.into_iter() {
        for (i, pixel) in layer.into_iter().enumerate() {
            match pixels.get_mut(i) {
                Some(list) => list.push(pixel),
                None => pixels.push(vec![pixel]),
            }
        }
    }

    pixels
        .into_iter()
        .map(|values| {
            for value in values {
                match value {
                    0 => return 0,
                    1 => return 1,
                    2 => {}
                    unknown => panic!("Unknown value {}", unknown),
                }
            }
            return 2;
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_image() -> Result<()> {
        let input = "123456789012";
        let output = parse_image(input, 3, 2)?;
        assert_eq!(output, vec![vec![1, 2, 3, 4, 5, 6], vec![7, 8, 9, 0, 1, 2]]);

        Ok(())
    }

    #[test]
    fn test_collapse_layers() -> Result<()> {
        let input = "0222112222120000";
        let image = parse_image(input, 2, 2)?;
        let output = collapse_layers(image);
        assert_eq!(output, vec![0, 1, 1, 0]);
        Ok(())
    }
}
