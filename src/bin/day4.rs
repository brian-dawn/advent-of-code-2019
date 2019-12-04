use itertools::Itertools;

const INPUT_START: i32 = 240298;
const INPUT_END: i32 = 784956;

fn valid(password: i32) -> bool {
    let mut previous = None;
    let mut n = password;
    let mut found_double = false;
    while n != 0 {
        let digit = n % 10;
        if let Some(prev) = previous {
            if prev < digit {
                return false;
            } else if prev == digit {
                found_double = true;
            }
        }

        previous = Some(digit);
        n = n / 10;
    }

    found_double
}

#[test]
fn test_valid() {
    assert!(valid(111111));
    assert!(valid(123455));
    assert!(valid(1123459));
    assert!(!valid(123456));
    assert!(!valid(123454));
    assert!(!valid(323454));
    assert!(!valid(123450));
}

fn valid_only_doubles(password: i32) -> bool {
    let mut n = password;
    let mut digits = Vec::new();
    while n != 0 {
        let digit = n % 10;
        digits.push(digit);
        n = n / 10;
    }

    let mut double_found = false;
    let mut previous = 99;
    for (digit, count) in digits
        .into_iter()
        .group_by(|i| *i)
        .into_iter()
        .map(|(key, grp)| (key, grp.count()))
    {
        if count == 2 {
            double_found = true;
        }
        if previous < digit {
            return false;
        }
        previous = digit;
    }
    return double_found;
}

#[test]
fn test_valid_only_doubles() {
    assert!(!valid_only_doubles(12345));
    assert!(valid_only_doubles(123455));
    assert!(valid_only_doubles(1123459));
    assert!(valid_only_doubles(111122));
    assert!(!valid_only_doubles(123444));
    assert!(!valid_only_doubles(111458));
    assert!(!valid_only_doubles(114583));
}

fn main() {
    let part1 = (INPUT_START..INPUT_END).filter(|i| valid(*i)).count();
    println!("part1: {}", part1);

    let part2 = (INPUT_START..INPUT_END)
        .filter(|i| valid_only_doubles(*i))
        .count();
    println!("part2: {}", part2);
}
