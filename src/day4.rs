use std::path::Path;

pub fn part1() -> usize {
    let path = Path::new("day4-input.txt");
    let input = std::fs::read_to_string(path).expect("read");
    let passports = input.as_str().split("\n\n");
    let all_keys = &["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
    passports
        .map(|passport| {
            all_keys
                .iter()
                .all(|k| get_keys(passport).find(|key| key == k).is_some())
        })
        .filter(|b| *b)
        .count()
}

fn get_keys(passport: &str) -> impl Iterator<Item = &str> {
    passport
        .split_whitespace()
        .map(|p| p.split(':').next().expect("next"))
}

pub fn part2() -> usize {
    let path = Path::new("day4-input.txt");
    let input = std::fs::read_to_string(path).expect("read");
    let passports = input.as_str().split("\n\n");
    let all_keys = &["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
    passports
        .map(|passport| {
            all_keys.iter().all(|k| {
                get_kvs(passport)
                    .find(|(key, val)| key == k && is_valid(key, val))
                    .is_some()
            })
        })
        .filter(|b| *b)
        .count()
}

fn get_kvs(passport: &str) -> impl Iterator<Item = (&str, &str)> {
    passport.split_whitespace().map(|p| {
        let mut s = p.split(':');
        (s.next().expect("first"), s.next().expect("second"))
    })
}

fn is_valid(key: &str, val: &str) -> bool {
    match key {
        "byr" => range_parse(val, 1920, 2002),
        "iyr" => range_parse(val, 2010, 2020),
        "eyr" => range_parse(val, 2020, 2030),
        "hgt" => match val.split_at(val.len() - 2) {
            (val, "cm") => range_parse(val, 150, 193),
            (val, "in") => range_parse(val, 59, 76),
            _ => false,
        },
        "hcl" => match val.split_at(1) {
            ("#", val) => hex_parse(val),
            _ => false,
        },
        "ecl" => matches!(val, "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth"),
        "pid" => pid_parse(val),
        _ => true,
    }
}

fn range_parse(input: &str, min: u32, max: u32) -> bool {
    let val: Result<u32, _> = input.parse();
    if let Ok(val) = val {
        min <= val && val <= max
    } else {
        false
    }
}

fn hex_parse(input: &str) -> bool {
    let allowed = vec![
        '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'a', 'b', 'c', 'd', 'e', 'f',
    ];
    input.chars().all(|c| allowed.contains(&c))
}

fn pid_parse(input: &str) -> bool {
    let allowed = vec!['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];
    input.len() == 9 && input.chars().all(|c| allowed.contains(&c))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(242, part1())
    }

    #[test]
    fn test_part2() {
        assert_eq!(186, part2())
    }
}
