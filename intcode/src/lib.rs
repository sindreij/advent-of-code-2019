pub mod computer;

pub use computer::{Computer, IO};

use anyhow::Result;

pub fn parse_program(program: &str) -> Result<Vec<i64>> {
    Ok(program
        .split(",")
        .filter(|s| !s.is_empty())
        .map(|val| Ok(val.trim().parse()?))
        .collect::<Result<Vec<i64>>>()?)
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
