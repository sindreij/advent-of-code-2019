mod ai;
mod ui;

use std::collections::HashSet;

use anyhow::Result;
use async_std::sync::Receiver;

use intcode::Computer;

#[async_std::main]
async fn main() -> Result<()> {
    let input = include_str!("../input/input.txt");
    // let mut input = String::new();
    // io::stdin().read_to_string(&mut input)?;

    println!("Part1: {}", part1(&input).await?);

    part2()?;

    Ok(())
}

#[derive(Debug)]
struct Tile {
    x: i64,
    y: i64,
    tile_id: i64,
}

async fn read_tile(stream: &Receiver<i64>) -> Option<Tile> {
    let x = stream.recv().await?;
    let y = stream.recv().await?;
    let tile_id = stream.recv().await?;
    Some(Tile { x, y, tile_id })
}

async fn part1(input: &str) -> Result<usize> {
    let mut computer = Computer::from_mem(intcode::parse_program(input)?);

    let output = computer.create_output_channel();

    let computer = computer.spawn();

    let mut tiles = HashSet::new();

    while let Some(tile) = read_tile(&output).await {
        if tile.tile_id == 2 {
            tiles.insert((tile.x, tile.y));
        } else {
            tiles.remove(&(tile.x, tile.y));
        }
    }

    computer.await?;

    println!("max X: {}", tiles.iter().map(|(x, _y)| x).max().unwrap());
    println!("min X: {}", tiles.iter().map(|(x, _y)| x).min().unwrap());
    println!("max Y: {}", tiles.iter().map(|(_x, y)| y).max().unwrap());
    println!("min Y: {}", tiles.iter().map(|(_x, y)| y).min().unwrap());

    Ok(tiles.len())
}

fn part2() -> Result<()> {
    ui::run()?;

    // let mut computer = Computer::from_mem(intcode::parse_program(input)?);

    // let output = computer.create_output_channel();
    // let input = computer.create_input_channel();

    // let computer = computer.spawn();

    // let mut tiles = HashSet::new();
    // let mut ball = (0, 0);
    // let mut paddle = (0, 0);

    // while let Some(tile) = read_tile(&output).await {
    //     if tile_id == 4 {

    //     }
    // }

    Ok(())
}
