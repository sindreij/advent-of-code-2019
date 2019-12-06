use std::collections::HashMap;
use std::io::{self, Read};

type Result<T> = ::std::result::Result<T, Box<dyn::std::error::Error>>;

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    println!("Part1: {}", part1(&input)?);
    println!("Part2: {}", part2(&input)?);

    Ok(())
}

fn parse_input(input: &str) -> Vec<(String, String)> {
    input
        .split('\n')
        .filter_map(|el| {
            let parts = el.split(')').map(|s| s.to_owned()).collect::<Vec<_>>();
            if parts.len() < 2 {
                None
            } else {
                Some((parts[0].clone(), parts[1].clone()))
            }
        })
        .collect()
}

fn part1(input: &str) -> Result<i32> {
    let mut children = HashMap::new();
    let mut parents = HashMap::new();

    let data = parse_input(input);

    for (parent, child) in data {
        children
            .entry(parent.clone())
            .or_insert_with(|| Vec::new())
            .push(child.clone());
        parents.insert(child, parent);
    }

    let mut orbits = HashMap::new();
    orbits.insert("COM".to_owned(), 0);

    let mut stack = children.get("COM").expect("NO COM").clone();

    while let Some(object) = stack.pop() {
        orbits.insert(object.clone(), orbits[&*parents[&*object]] + 1);
        if let Some(children) = children.get(&object) {
            stack.extend_from_slice(&children);
        }
    }

    Ok(orbits.values().sum())
}

fn part2(input: &str) -> Result<i32> {
    Ok(42)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_parse_inputs() -> Result<()> {
        assert_eq!(
            parse_input("COM)B\nB)C\nC)D\nD)E\n"),
            vec![("COM", "B"), ("B", "C"), ("C", "D"), ("D", "E")]
                .iter()
                .map(|(a, b)| (a.to_string(), b.to_string()))
                .collect::<Vec<_>>()
        );
        Ok(())
    }

    #[test]
    fn test_part_1() -> Result<()> {
        let input = include_str!("../input/basic.txt");
        assert_eq!(part1(input)?, 42);
        Ok(())
    }
}
