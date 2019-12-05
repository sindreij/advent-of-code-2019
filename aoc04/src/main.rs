type Result<T> = ::std::result::Result<T, Box<dyn::std::error::Error>>;

fn main() -> Result<()> {
    let input = (197487, 673251);
    println!("Part1: {}", part1(input));
    println!("Part2: {}", part2(input));

    Ok(())
}

fn part1((a, b): (i32, i32)) -> i32 {
    (a..=b).filter(|a| is_valid_password(*a)).count() as i32
}

fn part2((a, b): (i32, i32)) -> i32 {
    (a..=b).filter(|a| is_valid_password_2(*a)).count() as i32
}

fn is_valid_password(mut password: i32) -> bool {
    let mut digits = [0; 6];
    let base = 10;
    for i in 0..6 {
        digits[i] = password % base;
        password /= base;
    }
    let mut have_pair = false;
    for i in 0..5 {
        if digits[i] < digits[i + 1] {
            return false;
        }
        if digits[i] == digits[i + 1] {
            have_pair = true;
        }
    }
    have_pair
}

fn is_valid_password_2(mut password: i32) -> bool {
    let mut digits = [0; 6];
    let base = 10;
    for i in 0..6 {
        digits[i] = password % base;
        password /= base;
    }
    let mut have_pair = false;
    for i in 0..5 {
        if digits[i] < digits[i + 1] {
            return false;
        }
        if digits[i] == digits[i + 1]
            && (i == 4 || digits[i + 2] != digits[i])
            && (i == 0 || digits[i - 1] != digits[i])
        {
            have_pair = true;
        }
    }
    have_pair
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1((111111, 111111)), 1);
    }

    #[test]
    fn test_is_valid_password_2() {
        assert!(!is_valid_password_2(223450));
        assert!(!is_valid_password_2(123789));
        assert!(is_valid_password_2(112233));
        assert!(!is_valid_password_2(123444));
        assert!(is_valid_password_2(111122));
    }

    #[test]
    fn test_is_valid_password() {
        assert!(is_valid_password(111111));
        assert!(!is_valid_password(223450));
        assert!(!is_valid_password(123789));
    }
}
