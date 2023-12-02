use std::collections::HashMap;

use advent_of_code_2023::parse_args;
use color_eyre::Result;

static INPUT: &str = include_str!("../inputs/input-1");

fn part_one(input: &str) -> Result<u32> {
    let total = input
        .lines()
        .flat_map(|line| {
            let numbers = line
                .chars()
                .filter(|byte| byte.is_ascii_digit())
                .collect::<Vec<_>>();

            Some(numbers.first()?.to_digit(10)? * 10 + numbers.last()?.to_digit(10)?)
        })
        .sum();

    Ok(total)
}

fn part_two(input: &str) -> Result<u32> {
    let lookup = HashMap::from([
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ]);

    let mut total = 0;

    for line in input.lines() {
        let mut numbers = Vec::new();
        let mut word = String::new();

        for char in line.chars() {
            if char.is_ascii_digit() {
                numbers.push(char.to_digit(10).unwrap());
                continue;
            }

            if char.is_alphabetic() {
                word.push(char);

                // avoid double borrow on word with a normal loop
                for num in [
                    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
                ] {
                    if word.contains(num) {
                        word.clear();
                        word.push(char);
                        numbers.push(*lookup.get(num).unwrap());
                    }
                }
            }
        }

        total += numbers.first().unwrap() * 10 + numbers.last().unwrap();
    }

    Ok(total)
}

fn main() -> Result<()> {
    let args = parse_args();

    match args.part {
        1 => println!("{}", part_one(INPUT)?),
        2 => println!("{}", part_two(INPUT)?),
        _ => println!(
            "got unexpected value for --part: {} (try 1 or 2)",
            args.part
        ),
    };

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    static TEST_INPUT: &str = INPUT;

    #[test]
    fn test_one() -> Result<()> {
        assert_eq!(54573, part_one(TEST_INPUT)?);
        Ok(())
    }

    #[test]
    fn test_two() -> Result<()> {
        assert_eq!(54591, part_two(TEST_INPUT)?);
        Ok(())
    }
}
