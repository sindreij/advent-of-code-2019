use std::collections::HashSet;

use anyhow::{bail, Result};

use crate::utils::*;

pub fn calculate_equalibrium(moons: &[Moon]) -> Result<i64> {
    let mut moons = moons.to_vec();
    let mut states = HashSet::new();
    for i in 0..4686774925 {
        let new_moons = step(&moons);

        if !states.insert(moons) {
            return Ok(i);
        }
        moons = new_moons
    }

    bail!("Never got to the same state twice")
}

pub fn step(moons: &[Moon]) -> Vec<Moon> {
    moons.iter().map(|moon| step_one(moon, moons)).collect()
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
    fn test_equalibrium() -> Result<()> {
        let input = parse_input(include_str!("../input/test_input.txt"));
        assert_eq!(calculate_equalibrium(&input)?, 2772);
        Ok(())
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
