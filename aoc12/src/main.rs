mod fast;
mod slow;
mod utils;

use std::io::{self, Read};

use anyhow::Result;

use crate::utils::parse_input;

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    println!("Part1: {}", part1(&input)?);
    println!("Part1: {}", part2(&input)?);

    Ok(())
}

fn part1(input: &str) -> Result<i32> {
    let mut moons = parse_input(input);
    for _ in 0..1000 {
        moons = slow::step(&moons);
    }

    Ok(moons
        .iter()
        .map(|moon| moon.pos.energy() * moon.vel.energy())
        .sum())
}

fn part2(input: &str) -> Result<i64> {
    let moons = parse_input(input);
    fast::calculate_equalibrium(&moons)
}
