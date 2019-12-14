use anyhow::{bail, Result};
use num::Integer;

use crate::utils::{unit_one, Moon};

#[derive(Debug, Eq, PartialEq, Clone, Hash)]
struct MoonAxis {
    pos: i32,
    vel: i32,
}

fn split_moons(moons: &[Moon]) -> Vec<Vec<MoonAxis>> {
    vec![
        // x
        moons
            .iter()
            .map(|moon| MoonAxis {
                pos: moon.pos.x,
                vel: moon.vel.x,
            })
            .collect(),
        // y
        moons
            .iter()
            .map(|moon| MoonAxis {
                pos: moon.pos.y,
                vel: moon.vel.y,
            })
            .collect(),
        // z
        moons
            .iter()
            .map(|moon| MoonAxis {
                pos: moon.pos.z,
                vel: moon.vel.z,
            })
            .collect(),
    ]
}

fn least_common_multiple(numbers: &[i64]) -> i64 {
    numbers.iter().fold(numbers[0], |a, b| a.lcm(b))
}

pub fn calculate_equalibrium(input: &[Moon]) -> Result<i64> {
    Ok(least_common_multiple(
        &split_moons(input)
            .iter()
            .map(|axis| calculate_equalibrium_axis(axis))
            .collect::<Result<Vec<_>>>()?,
    ))
}

fn calculate_equalibrium_axis(moons: &[MoonAxis]) -> Result<i64> {
    let mut moons = moons.to_vec();
    let start_state = moons.clone();
    for i in 0..4686774925 {
        moons = step(&moons);

        if moons == start_state {
            return Ok(i);
        }
    }

    bail!("Never got to the same state twice")
}

fn step_one(moon: &MoonAxis, moons: &[MoonAxis]) -> MoonAxis {
    let mut moon = moon.clone();
    for second in moons {
        if *second == moon {
            // TODO: What about a moon at exactly the same position
            continue;
        }
        moon.vel += unit_one(second.pos, moon.pos);
    }
    moon.pos += moon.vel;
    moon
}

fn step(moons: &[MoonAxis]) -> Vec<MoonAxis> {
    moons.iter().map(|moon| step_one(moon, moons)).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::parse_input;

    #[test]
    fn test_lcm() {
        assert_eq!(least_common_multiple(&[75, 30]), 150);
        assert_eq!(least_common_multiple(&[75, 30, 12]), 300);
    }

    #[test]
    fn test_split_moons() -> Result<()> {
        let input = "<x=-1, y=0, z=2>\n<x=2, y=-10, z=-7>\n";
        let moons = parse_input(input);

        let splitted = split_moons(&moons);

        assert_eq!(
            splitted,
            vec![
                vec![MoonAxis { pos: -1, vel: 0 }, MoonAxis { pos: 2, vel: 0 }],
                vec![MoonAxis { pos: 0, vel: 0 }, MoonAxis { pos: -10, vel: 0 }],
                vec![MoonAxis { pos: 2, vel: 0 }, MoonAxis { pos: -7, vel: 0 }],
            ]
        );

        Ok(())
    }

    #[test]
    fn test_equalibrium() -> Result<()> {
        assert_eq!(
            calculate_equalibrium(&parse_input(include_str!("../input/test_input.txt")))?,
            2772
        );
        assert_eq!(
            calculate_equalibrium(&parse_input(include_str!("../input/second.txt")))?,
            4686774924
        );
        Ok(())
    }

    #[test]
    fn test_step_one() {
        let moons_axis =
            split_moons(&parse_input(include_str!("../input/test_input.txt")))[0].clone();
        assert_eq!(moons_axis[0], MoonAxis { pos: -1, vel: 0 });
        let after = step_one(&moons_axis[0], &moons_axis);
        assert_eq!(after, MoonAxis { pos: 2, vel: 3 });

        assert_eq!(
            step_one(&moons_axis[1], &moons_axis),
            MoonAxis { pos: 3, vel: 1 }
        );

        assert_eq!(
            step_one(&moons_axis[2], &moons_axis),
            MoonAxis { pos: 1, vel: -3 }
        );
    }
}
