use std::io::{self, Read};

type Result<T> = ::std::result::Result<T, Box<dyn::std::error::Error>>;

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    println!("Part1: {}", part1(&input)?);
    println!("Part2: {}", part2(&input)?);

    Ok(())
}

fn part1(input: &str) -> Result<isize> {
    let mut data = input
        .split(",")
        .map(|val| Ok(dbg!(val).parse()?))
        .collect::<Result<Vec<isize>>>()?;

    data[1] = 12;
    data[2] = 2;

    run_program(data)
}

fn run_program(mut data: Vec<isize>) -> Result<isize> {
    let mut pc = 0;

    loop {
        let opcode = data[pc];
        if opcode == 99 {
            return Ok(data[0]);
        }
        let val1 = data[data[pc + 1] as usize];
        let val2 = data[data[pc + 2] as usize];
        let register = data[pc + 3];
        let result = match opcode {
            1 => val1 + val2,
            2 => val1 * val2,
            _ => Err(format!("Unknown opcode, {}", opcode))?,
        };
        println!("Inserting {} into {}", result, register);
        data[register as usize] = result;

        pc += 4;
    }
}

fn part2(input: &str) -> Result<isize> {
    Ok(0)
}

#[cfg(test)]
mod tests_part1 {
    use super::*;

    #[test]
    fn test_simple() -> Result<()> {
        assert_eq!(run_program(vec![1, 0, 0, 0, 99])?, 2);
        assert_eq!(run_program(vec![2, 3, 0, 3, 99])?, 2);
        assert_eq!(run_program(vec![2, 4, 4, 5, 99, 0])?, 2);
        assert_eq!(run_program(vec![1, 1, 1, 4, 99, 5, 6, 0, 99])?, 30);
        Ok(())
    }
}

#[cfg(test)]
mod tests_part2 {
    use super::*;
}
