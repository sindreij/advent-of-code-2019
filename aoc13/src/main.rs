use std::collections::HashSet;
use std::io::{self, Read};

use anyhow::Result;
use async_std::sync::Receiver;

use intcode::Computer;

#[async_std::main]
async fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    println!("Part1: {}", part1(&input).await?);

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

    let computer = computer.spawn().await;

    let mut tiles = HashSet::new();

    while let Some(tile) = read_tile(&output).await {
        if tile.tile_id == 2 {
            tiles.insert((tile.x, tile.y));
        } else {
            tiles.remove(&(tile.x, tile.y));
        }
    }

    computer.await?;

    Ok(tiles.len())
}
