mod computer;

use std::io::{self, Read};

use computer::Computer;

type Result<T> = ::std::result::Result<T, Box<dyn::std::error::Error>>;

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    println!("Part1: {}", part1(&input)?);
    println!("Part2: {}", part2(&input)?);

    Ok(())
}

fn parse_program(program: &str) -> Result<Vec<i32>> {
    Ok(program
        .split(",")
        .map(|val| Ok(val.parse()?))
        .collect::<Result<Vec<i32>>>()?)
}

fn part1(input: &str) -> Result<i32> {
    let mut computer = Computer::from_mem(parse_program(input)?);
    computer.run();
    Ok(12)
}

fn part2(input: &str) -> Result<i32> {
    Ok(12)
}
