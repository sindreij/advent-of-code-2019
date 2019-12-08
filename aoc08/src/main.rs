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
    Ok(-1)
}

fn parse_image(data: &str, width: usize, height: usize) -> Result<Vec<Vec<i32>>> {
    let data = data
        .chars()
        .filter_map(|a| a.to_string().parse().ok())
        .collect::<Vec<i32>>();
    assert_eq!(data.len() % (width * height), 0);

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
}
