use std::collections::{BTreeMap, BTreeSet};
use std::io::{self, Read};

use anyhow::Result;
use maplit::btreeset;

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    // println!("Part1: {}", part1(&input));
    println!("Part2: {}", part2(&input));

    Ok(())
}

fn part1(input: &str) -> usize {
    let map = parse_input(input);
    map.iter()
        .map(|curr| get_visible(*curr, &map))
        .max()
        .unwrap()
}

#[derive(Debug)]
struct Astroid {
    x: i32,
    y: i32,
    // dist squared
    dist: i32,
    angle: i32,
}

fn part2(input: &str) -> i32 {
    let map = parse_input(input);
    let (x, y) = map
        .iter()
        .max_by_key(|curr| get_visible(**curr, &map))
        .cloned()
        .unwrap();

    let positions = map
        .iter()
        .filter(|astroid| **astroid != (x, y))
        .map(|(x2, y2)| Astroid {
            x: *x2,
            y: *y2,
            dist: (y - y2).pow(2) + (x - x2).pow(2),
            angle: vaporized_angle((x, y), (*x2, *y2)),
        })
        .collect::<Vec<_>>();

    // positions.sort();

    let mut per_parsec = BTreeMap::new();

    for astroid in positions {
        let astroids = per_parsec.entry(astroid.angle).or_insert(vec![]);
        astroids.push(astroid);
        astroids.sort_by_key(|a| -a.dist);
    }

    let keys = per_parsec.keys().copied().collect::<Vec<_>>();

    let mut vaporized = 0;
    loop {
        let start_vaporized = vaporized;
        for angle in &keys {
            let astroids = per_parsec.get_mut(&angle).unwrap();
            if let Some(astroid) = astroids.pop() {
                dbg!(&astroid);
                vaporized += 1;
                if vaporized == 200 {
                    return astroid.x * 100 + astroid.y;
                }
            }
        }
        if start_vaporized == vaporized {
            break;
        }
    }

    vaporized
}

fn vaporized_angle((x1, y1): (i32, i32), (x2, y2): (i32, i32)) -> i32 {
    (((-x1 + x2) as f64).atan2((-y1 + y2) as f64) * 1000000.) as i32 * -1
}

fn parse_input(input: &str) -> BTreeSet<(i32, i32)> {
    input
        .split("\n")
        .enumerate()
        .map(|(y, line)| {
            line.chars()
                .enumerate()
                .filter(|(_, ch)| *ch == '#')
                .map(move |(x, _)| (x as i32, y as i32))
        })
        .flatten()
        .collect()
}

fn get_visible((x, y): (i32, i32), map: &BTreeSet<(i32, i32)>) -> usize {
    map.iter()
        .filter(|astroid| **astroid != (x, y))
        .map(|(x2, y2)| (((y - y2) as f64).atan2((x - x2) as f64) * 1000000.) as i32)
        .collect::<BTreeSet<_>>()
        .len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vaporized_angle() {
        assert_eq!(vaporized_angle((8, 3), (7, 1)), 2677945);
        assert_eq!(vaporized_angle((8, 3), (8, 1)), -3141592);
        assert_eq!(vaporized_angle((8, 3), (10, 3)), -1570796);
    }

    #[test]
    fn test_parsing() {
        let map = include_str!("../input/map1.txt");
        let output = btreeset![
            (1, 0),
            (4, 0),
            (0, 2),
            (1, 2),
            (2, 2),
            (3, 2),
            (4, 2),
            (4, 3),
            (3, 4),
            (4, 4),
        ];
        assert_eq!(parse_input(map), output);
    }

    #[test]
    fn test_get_visible() {
        let map = parse_input(include_str!("../input/map1.txt"));
        assert_eq!(get_visible((3, 4), &map), 8);
        assert_eq!(get_visible((1, 0), &map), 7);
        assert_eq!(get_visible((0, 2), &map), 6);
        assert_eq!(get_visible((1, 2), &map), 7);
        assert_eq!(get_visible((2, 2), &map), 7);
        assert_eq!(get_visible((3, 2), &map), 7);

        let map5 = parse_input(include_str!("../input/map5.txt"));
        assert_eq!(get_visible((11, 13), &map5), 210);
    }
}
