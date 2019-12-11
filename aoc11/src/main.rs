use std::collections::BTreeMap;
use std::io::{self, prelude::*};

use anyhow::{bail, Result};
use async_trait::async_trait;

use intcode::{self, Computer, IO};

#[derive(Debug, Clone)]
struct Locomotion {
    x: i32,
    y: i32,
    pixels: BTreeMap<(i32, i32), Color>,
    direction: Direction,
    next_instruction: LocomotionInstruction,
}

#[derive(Debug, Clone)]
enum LocomotionInstruction {
    Paint,
    Move,
}

impl Locomotion {
    fn new() -> Self {
        Locomotion {
            x: 0,
            y: 0,
            pixels: BTreeMap::new(),
            direction: Direction::Forward,
            next_instruction: LocomotionInstruction::Paint,
        }
    }

    fn painted_once(&self) -> usize {
        self.pixels.len()
    }

    fn paint(&mut self, color: Color) {
        self.pixels.insert((self.x, self.y), color);
    }

    fn get(&self, x: i32, y: i32) -> Color {
        self.pixels.get(&(x, y)).cloned().unwrap_or(Color::Black)
    }

    fn turn(&mut self, turn: Turn) {
        use Direction::*;
        self.direction = match turn {
            Turn::Left => match self.direction {
                Left => Backward,
                Backward => Right,
                Right => Forward,
                Forward => Left,
            },
            Turn::Right => match self.direction {
                Left => Forward,
                Forward => Right,
                Right => Backward,
                Backward => Left,
            },
        }
    }
    fn move_forwards(&mut self) {
        use Direction::*;
        let (x, y) = (self.x, self.y);
        let (x, y) = match self.direction {
            Forward => (x, y - 1),
            Backward => (x, y + 1),
            Left => (x - 1, y),
            Right => (x + 1, y),
        };
        self.x = x;
        self.y = y;
    }
}

#[async_trait]
impl intcode::IO for Locomotion {
    async fn input(&self) -> Result<i64> {
        Ok(match self.get(self.x, self.y) {
            Color::White => 1,
            Color::Black => 0,
        })
    }
    async fn output(&mut self, data: i64) -> Result<()> {
        use LocomotionInstruction::*;
        self.next_instruction = match self.next_instruction {
            Paint => {
                self.paint(match data {
                    0 => Color::Black,
                    1 => Color::White,
                    color => bail!("Unknown color {}", color),
                });
                Move
            }
            Move => {
                self.turn(match data {
                    0 => Turn::Left,
                    1 => Turn::Right,
                    _ => bail!("Uknown turn {}", data),
                });
                self.move_forwards();
                Paint
            }
        };

        Ok(())
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
enum Direction {
    Left,
    Right,
    Forward,
    Backward,
}

#[derive(Debug, Clone, Eq, PartialEq)]
enum Color {
    White,
    Black,
}

#[derive(Debug, Clone, Eq, PartialEq)]
enum Turn {
    Left,
    Right,
}

#[async_std::main]
async fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    println!("Part1: {}", part1(&input).await?);

    Ok(())
}

async fn part1(input: &str) -> Result<usize> {
    let mut computer =
        Computer::from_mem(intcode::parse_program(input)?).with_io(Locomotion::new());

    computer.run().await?;

    Ok(computer.into_io().painted_once())
}

#[cfg(test)]
mod tests {
    use super::*;
    use Color::*;
    #[test]
    fn test_locomotion() {
        let mut loco = Locomotion::new();
        loco.paint(White);
        assert_eq!(loco.get(0, 0), White);
        assert_eq!(loco.get(-1, 0), Black);
        loco.turn(Turn::Left);
        loco.move_forwards();
        loco.paint(White);
        assert_eq!(loco.get(-1, 0), White);
        loco.turn(Turn::Right);
        loco.move_forwards();
        loco.paint(White);
        assert_eq!(loco.get(-1, -1), White);
        loco.paint(Black);
        assert_eq!(loco.get(-1, -1), Black);
        loco.move_forwards();
        loco.paint(White);
        assert_eq!(loco.get(-1, -2), White);
    }

    #[async_std::test]
    async fn test_with_computersignals() -> Result<()> {
        let mut loco = Locomotion::new();
        assert_eq!(loco.input().await?, 0);
        assert_eq!(loco.input().await?, 0);
        // Paint White
        loco.output(1).await?;
        assert_eq!(loco.input().await?, 1);
        // Turn Left
        loco.output(0).await?;
        assert_eq!(loco.input().await?, 0);
        // Paint Black
        loco.output(0).await?;
        assert_eq!(loco.input().await?, 0);
        // Turn Left
        loco.output(0).await?;

        loco.output(1).await?;
        loco.output(0).await?;
        loco.output(1).await?;
        loco.output(0).await?;
        assert_eq!(loco.input().await?, 1);

        loco.output(0).await?;
        loco.output(1).await?;
        loco.output(1).await?;
        loco.output(0).await?;
        loco.output(1).await?;
        loco.output(0).await?;

        assert_eq!(loco.painted_once(), 6);

        Ok(())
    }
}
