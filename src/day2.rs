// --- Day 2: Password Philosophy ---

// Your flight departs in a few days from the coastal airport; the
// easiest way down to the coast from here is via toboggan.

// The shopkeeper at the North Pole Toboggan Rental Shop is having a
// bad day. "Something's wrong with our computers; we can't log in!"
// You ask if you can take a look.

// Their password database seems to be a little corrupted: some of the
// passwords wouldn't have been allowed by the Official Toboggan
// Corporate Policy that was in effect when they were chosen.

// To try to debug the problem, they have created a list (your puzzle
// input) of passwords (according to the corrupted database) and the
// corporate policy when that password was set.

// For example, suppose you have the following list:

// 1-3 a: abcde
// 1-3 b: cdefg
// 2-9 c: ccccccccc

// Each line gives the password policy and then the password. The
// password policy indicates the lowest and highest number of times a
// given letter must appear for the password to be valid. For example,
// 1-3 a means that the password must contain a at least 1 time and at
// most 3 times.

// In the above example, 2 passwords are valid. The middle password,
// cdefg, is not; it contains no instances of b, but needs at least
// 1. The first and third passwords are valid: they contain one a or
// nine c, both within the limits of their respective policies.

// How many passwords are valid according to their policies?

use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

struct Entry {
    min: usize,
    max: usize,
    character: char,
    password: String,
}

impl std::str::FromStr for Entry {
    type Err = String;
    fn from_str(raw: &str) -> Result<Self, String> {
        let words: Vec<_> = raw.split_whitespace().collect();
        let (rangestr, charstr, pwdstr) = (words[0], words[1], words[2]);
        let ranges: Vec<_> = rangestr.split('-').collect();
        Ok(Entry {
            min: ranges[0].parse().expect("min"),
            max: ranges[1].parse().expect("max"),
            character: charstr.chars().next().expect("char"),
            password: pwdstr.into(),
        })
    }
}

fn is_valid(
    Entry {
        min,
        max,
        character,
        password,
    }: &Entry,
) -> bool {
    let count = password.as_str().chars().filter(|c| c == character).count();
    count >= *min && count <= *max
}

pub fn part1() -> usize {
    let file = File::open(Path::new("day2-input.txt")).expect("open");
    let mut entries: Vec<Entry> = Vec::new();
    for line in io::BufReader::new(file).lines() {
        entries.push(line.expect("line").parse().expect("parse"))
    }
    entries.iter().filter(|e| is_valid(e)).count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(445, part1())
    }
}
