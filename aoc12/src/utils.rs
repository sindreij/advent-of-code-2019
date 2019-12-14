use std::ops::{Add, AddAssign};

use regex::Regex;

pub fn parse_input(input: &str) -> Vec<Moon> {
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

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct Ptr {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

impl Ptr {
    pub fn null() -> Self {
        Self { x: 0, y: 0, z: 0 }
    }

    pub fn energy(&self) -> i32 {
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

#[derive(Debug, Eq, PartialEq, Clone, Hash)]
pub struct Moon {
    pub pos: Ptr,
    pub vel: Ptr,
}

pub fn unit_one(a: i32, b: i32) -> i32 {
    use std::cmp::Ordering::*;
    match a.cmp(&b) {
        Less => -1,
        Equal => 0,
        Greater => 1,
    }
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
}
