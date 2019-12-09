mod computer;

use std::io::{self, Read};

use anyhow::{anyhow, Result};
use async_std::{sync::channel, task};
use futures::future::join_all;
use itertools::Itertools;

use computer::Computer;

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    println!("Part1: {}", task::block_on(part1(&input))?);
    println!("Part2: {}", task::block_on(part2(&input))?);

    Ok(())
}

fn parse_program(program: &str) -> Result<Vec<i64>> {
    Ok(program
        .split(",")
        .filter(|s| !s.is_empty())
        .map(|val| Ok(val.trim().parse()?))
        .collect::<Result<Vec<i64>>>()?)
}

async fn part1(input: &str) -> Result<i64> {
    let mut computer = Computer::from_mem(parse_program(input)?);
    computer.run().await?;
    Ok(-1)
}

async fn part2(input: &str) -> Result<i64> {
    Ok(-1)
}

#[cfg(test)]
mod tests {
    use super::*;
}
