use std::io::{self, Read};

type Result<T> = ::std::result::Result<T, Box<dyn::std::error::Error>>;

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    println!("Part1: {}", part1(&input)?);
    println!("Part2: {}", part2(&input)?);

    Ok(())
}

fn part1(input: &str) -> Result<i32> {
    Ok(input
        .split("\n")
        .filter(|value| !value.is_empty())
        .map(|input| {
            let value: i32 = input.parse()?;
            Ok(calculate_fuel(value))
        })
        .collect::<Result<Vec<_>>>()?
        .iter()
        .sum())
}

fn calculate_fuel(mass: i32) -> i32 {
    ((mass / 3) as i32) - 2
}

fn part2(input: &str) -> Result<i32> {
    Ok(input
        .split("\n")
        .filter(|value| !value.is_empty())
        .map(|input| {
            let mut value: i32 = input.parse()?;

            let mut total_fuel = 0;

            loop {
                let new_fuel = calculate_fuel(value);
                if new_fuel <= 0 {
                    break;
                }
                total_fuel += new_fuel;
                value = new_fuel
            }

            Ok(total_fuel)
        })
        .collect::<Result<Vec<_>>>()?
        .iter()
        .sum())
}

#[cfg(test)]
mod tests_part2 {
    use super::*;

    #[test]
    fn test_onemass() -> Result<()> {
        assert_eq!(part2("14")?, 2);
        assert_eq!(part2("1969")?, 966);
        assert_eq!(part2("100756")?, 50346);
        Ok(())
    }

    #[test]
    fn test_multiplemasses() -> Result<()> {
        assert_eq!(part2("14\n1969\n")?, 2 + 966);
        Ok(())
    }
}

#[cfg(test)]
mod tests_part1 {
    use super::*;

    #[test]
    fn test_onemass() -> Result<()> {
        assert_eq!(part1("12")?, 2);
        assert_eq!(part1("14")?, 2);
        assert_eq!(part1("1969")?, 654);
        assert_eq!(part1("100756")?, 33583);
        Ok(())
    }

    #[test]
    fn test_multiplemasses() -> Result<()> {
        assert_eq!(part1("12\n14\n")?, 2 + 2);
        assert_eq!(part1("12\n14\n1969\n")?, 2 + 2 + 654);
        Ok(())
    }
}
