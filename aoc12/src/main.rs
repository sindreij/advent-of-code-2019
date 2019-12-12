use std::collections::HashMap;
use std::io::{self, Read};
use std::ops::{Add, AddAssign};

use anyhow::Result;
use regex::Regex;

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    println!("Part1: {}", part1(&input)?);

    Ok(())
}

fn part1(input: &str) -> Result<i32> {
    let mut moons = parse_input(input);
    for _ in 0..1000 {
        moons = step(&moons);
    }

    Ok(moons
        .iter()
        .map(|moon| moon.pos.energy() * moon.vel.energy())
        .sum())
}

fn step(moons: &[Moon]) -> Vec<Moon> {
    moons.iter().map(|moon| step_one(moon, moons)).collect()
}

fn parse_input(input: &str) -> Vec<Moon> {
    let re = Regex::new(r"<x=(-?\d*), y=(-?\d*), z=(-?\d*)>").unwrap();

    input
        .split('\n')
        .filter(|s| !s.is_empty())
        .filter_map(|line| {
            let groups = re.captures(line)?;
            Some(Moon {
                pos: Ptr {
                    x: groups[1].parse().unwrap(),
                    y: groups[2].parse().unwrap(),
                    z: groups[3].parse().unwrap(),
                },
                vel: Ptr::null(),
            })
        })
        .collect()
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct Ptr {
    x: i32,
    y: i32,
    z: i32,
}

impl Ptr {
    fn null() -> Self {
        Self { x: 0, y: 0, z: 0 }
    }

    fn energy(&self) -> i32 {
        self.x.abs() + self.y.abs() + self.z.abs()
    }
}

impl Add<Ptr> for Ptr {
    type Output = Ptr;
    fn add(self, other: Ptr) -> Ptr {
        Ptr {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl AddAssign for Ptr {
    fn add_assign(&mut self, other: Self) {
        *self = *self + other;
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
struct Moon {
    pos: Ptr,
    vel: Ptr,
}

fn unit_one(a: i32, b: i32) -> i32 {
    use std::cmp::Ordering::*;
    match a.cmp(&b) {
        Less => -1,
        Equal => 0,
        Greater => 1,
    }
}

fn step_one(moon: &Moon, moons: &[Moon]) -> Moon {
    let mut moon = moon.clone();
    for second in moons {
        if *second == moon {
            // TODO: What about a moon at exactly the same position
            continue;
        }
        moon.vel.x += unit_one(second.pos.x, moon.pos.x);
        moon.vel.y += unit_one(second.pos.y, moon.pos.y);
        moon.vel.z += unit_one(second.pos.z, moon.pos.z);
    }
    moon.pos += moon.vel;
    moon
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let input = "<x=-1, y=0, z=2>\n<x=2, y=-10, z=-7>\n";
        let output = parse_input(input);
        assert_eq!(output[0].pos, Ptr { x: -1, y: 0, z: 2 });
        assert_eq!(
            output[1].pos,
            Ptr {
                x: 2,
                y: -10,
                z: -7
            }
        );
    }

    #[test]
    fn test_parse_2() {
        let input = include_str!("../input/test_input.txt");
        let output = parse_input(input);
        assert_eq!(output[0].pos, Ptr { x: -1, y: 0, z: 2 });
    }

    #[test]
    fn test_step_one() {
        let moons = parse_input(include_str!("../input/test_input.txt"));
        assert_eq!(
            moons[0],
            Moon {
                pos: Ptr { x: -1, y: 0, z: 2 },
                vel: Ptr { x: 0, y: 0, z: 0 }
            }
        );
        let after = step_one(&moons[0], &moons);
        assert_eq!(
            after,
            Moon {
                pos: Ptr { x: 2, y: -1, z: 1 },
                vel: Ptr { x: 3, y: -1, z: -1 }
            }
        );

        assert_eq!(
            step_one(&moons[1], &moons),
            Moon {
                pos: Ptr { x: 3, y: -7, z: -4 },
                vel: Ptr { x: 1, y: 3, z: 3 }
            }
        );

        assert_eq!(
            step_one(&moons[2], &moons),
            Moon {
                pos: Ptr { x: 1, y: -7, z: 5 },
                vel: Ptr { x: -3, y: 1, z: -3 }
            }
        );
    }
}
