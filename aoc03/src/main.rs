use std::io::{self, Read};

type Result<T> = ::std::result::Result<T, Box<dyn::std::error::Error>>;

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    println!("Part1: {}", part1(&input)?);
    println!("Part2: {}", part2(&input)?);

    Ok(())
}

#[derive(Debug, Eq, PartialEq)]
enum PathDesc {
    R(i32),
    U(i32),
    L(i32),
    D(i32),
}

fn get_path(desc: Vec<PathDesc>) -> Vec<(i32, i32)> {
    let mut result = vec![];

    let mut current = (0, 0);

    for curr in desc {
        let (distance, delta_x, delta_y) = match curr {
            PathDesc::R(distance) => (distance, 1, 0),
            PathDesc::L(distance) => (distance, -1, 0),
            PathDesc::U(distance) => (distance, 0, 1),
            PathDesc::D(distance) => (distance, 0, -1),
        };

        let (mut curr_x, mut curr_y) = current;

        for _ in 0..distance {
            curr_x += delta_x;
            curr_y += delta_y;
            result.push((curr_x, curr_y));
        }
        current = (curr_x, curr_y);
    }

    result
}

fn parse_path(input: &str) -> Result<Vec<PathDesc>> {
    input
        .split(",")
        .map(|input| {
            let distance = input[1..].parse()?;
            match &input[0..1] {
                "R" => Ok(PathDesc::R(distance)),
                "L" => Ok(PathDesc::L(distance)),
                "U" => Ok(PathDesc::U(distance)),
                "D" => Ok(PathDesc::D(distance)),
                bad_specifier => Err(format!("Unknown specifier {}", bad_specifier))?,
            }
        })
        .collect()
}

fn find_overlapps(path1: Vec<(i32, i32)>, path2: Vec<(i32, i32)>) -> Vec<(i32, i32)> {
    let mut result = vec![];
    for point1 in &path1 {
        for point2 in &path2 {
            if point1 == point2 {
                result.push(*point1);
            }
        }
    }
    result
}

fn manhattan_distance((x, y): (i32, i32)) -> i32 {
    x.abs() + y.abs()
}

fn find_closest(points: Vec<(i32, i32)>) -> Result<(i32, i32)> {
    Ok(points
        .iter()
        .min_by_key(|el| manhattan_distance(**el))
        .copied()
        .ok_or("No points given to find_closest")?)
}

fn part1(input: &str) -> Result<i32> {
    let parts = input.split("\n").collect::<Vec<_>>();
    let a = parts[0];
    let b = parts[1];

    let a_path = get_path(parse_path(a)?);
    let b_path = get_path(parse_path(b)?);
    let overlapps = dbg!(find_overlapps(a_path, b_path));

    Ok(manhattan_distance(find_closest(overlapps)?))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_closest_none() {
        assert_eq!(
            find_closest(vec![]).unwrap_err().to_string(),
            "No points given to find_closest"
        )
    }

    #[test]
    fn test_find_closest() -> Result<()> {
        assert_eq!(find_closest(vec![(0, 0)])?, (0, 0));
        assert_eq!(find_closest(vec![(1, 0), (0, 0)])?, (0, 0));
        Ok(())
    }

    #[test]
    fn test_manhattan_distance() {
        assert_eq!(manhattan_distance((0, 0)), 0);
        assert_eq!(manhattan_distance((0, 1)), 1);
        assert_eq!(manhattan_distance((4, 3)), 7);
        assert_eq!(manhattan_distance((-4, 3)), 7);
        assert_eq!(manhattan_distance((-4, -3)), 7);
        assert_eq!(manhattan_distance((4, -3)), 7);
    }

    #[test]
    fn test_find_overlapps() {
        assert_eq!(find_overlapps(vec![(0, 1)], vec![(0, 1)]), vec![(0, 1)]);
        assert_eq!(find_overlapps(vec![(0, 1)], vec![(0, 2)]), vec![]);
        assert_eq!(
            find_overlapps(vec![(1, 0), (0, 4)], vec![(0, 2), (0, 3), (0, 4)]),
            vec![(0, 4)]
        );

        assert_eq!(
            find_overlapps(
                vec![(1, 0), (0, 4), (0, 9), (1, 7)],
                vec![(0, 2), (0, 3), (0, 4), (1, 7), (1, 10)]
            ),
            vec![(0, 4), (1, 7)]
        );
    }

    #[test]
    fn test_parse_path_single() -> Result<()> {
        use PathDesc::*;
        assert_eq!(parse_path("R2")?, vec![R(2)]);
        assert_eq!(parse_path("R3")?, vec![R(3)]);
        assert_eq!(parse_path("R4")?, vec![R(4)]);
        assert_eq!(parse_path("L9")?, vec![L(9)]);
        assert_eq!(parse_path("U9")?, vec![U(9)]);
        assert_eq!(parse_path("D9")?, vec![D(9)]);
        Ok(())
    }

    #[test]
    fn test_parse_path_single_multiple() -> Result<()> {
        use PathDesc::*;
        assert_eq!(parse_path("R2,R3,L9,D8")?, vec![R(2), R(3), L(9), D(8)]);
        Ok(())
    }

    #[test]
    fn test_get_path_one() {
        use PathDesc::*;
        assert_eq!(get_path(vec![R(2)]), vec![(1, 0), (2, 0)]);
        assert_eq!(get_path(vec![R(3)]), vec![(1, 0), (2, 0), (3, 0)]);
        assert_eq!(get_path(vec![L(3)]), vec![(-1, 0), (-2, 0), (-3, 0)]);
        assert_eq!(get_path(vec![U(3)]), vec![(0, 1), (0, 2), (0, 3)]);
        assert_eq!(get_path(vec![D(3)]), vec![(0, -1), (0, -2), (0, -3)]);
    }

    #[test]
    fn test_get_path_multiple() {
        use PathDesc::*;

        assert_eq!(get_path(vec![R(1), R(1)]), vec![(1, 0), (2, 0)]);
        assert_eq!(get_path(vec![R(1), U(1)]), vec![(1, 0), (1, 1)]);
        assert_eq!(
            get_path(vec![R(2), U(3)]),
            vec![(1, 0), (2, 0), (2, 1), (2, 2), (2, 3)]
        );
    }
}

fn part2(input: &str) -> Result<isize> {
    Ok(0)
}
