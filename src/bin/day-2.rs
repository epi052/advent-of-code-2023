use std::collections::HashMap;
use std::str::FromStr;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::digit1,
    combinator::map_res,
    multi::many1,
    sequence::{delimited, pair},
    IResult,
};

use advent_of_code_2023::parse_args;
use color_eyre::Result;

static INPUT: &str = include_str!("../inputs/input-2");

fn parse_game_number(input: &str) -> IResult<&str, i32> {
    // goal: 'Game 1: ' -> returns 1 as u32

    // delimited returns the value of the parser in the middle, and ignores the parsers on the outside
    // map_res converts the value returned by the parser (1st param) to the type specified in the function (2nd param)
    delimited(tag("Game "), map_res(digit1, i32::from_str), tag(": "))(input)
}

fn parse_colors(input: &str) -> IResult<&str, Vec<(i32, &str)>> {
    // goal: '3 blue' -> returns 3 as u8
    // needs to handle '1 blue, ' '1 blue', and '1 blue; '
    // needs to handle red/green/blue as well

    many1(pair(
        map_res(digit1, i32::from_str),
        // alt returns the first successful parser
        delimited(
            tag(" "),
            alt((tag("blue"), tag("red"), tag("green"))),
            alt((tag(", "), tag("; "), tag(""))),
        ),
    ))(input)
}

fn part_one(input: &str) -> IResult<&str, i32> {
    let mut total = 0;
    let max_lookup = HashMap::from([("red", 12), ("green", 13), ("blue", 14)]);

    for line in input.lines() {
        let (line, game_number) = parse_game_number(line)?;
        let (_, colors) = parse_colors(line)?;

        if colors.iter().all(|(count, color)| {
            let max = max_lookup.get(color).unwrap();
            count <= max
        }) {
            total += game_number;
        }
    }

    Ok(("", total))
}

fn part_two(input: &str) -> IResult<&str, i32> {
    let mut total = 0;

    for line in input.lines() {
        let (line, _) = parse_game_number(line)?;
        let (_, colors) = parse_colors(line)?;
        let mut blue = 0;
        let mut red = 0;
        let mut green = 0;

        // get the largest number of each color
        for (count, color) in colors {
            match color {
                "blue" => blue = blue.max(count),
                "red" => red = red.max(count),
                "green" => green = green.max(count),
                _ => unreachable!(),
            }
        }
        total += blue * red * green;
    }

    Ok(("", total))
}

fn main() -> Result<()> {
    let args = parse_args();

    match args.part {
        1 => println!("{:?}", part_one(INPUT)?),
        2 => println!("{:?}", part_two(INPUT)?),
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
    static TEST_INPUT: &str = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue; 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

    #[test]
    fn test_one() -> Result<()> {
        // 2149 for part 1
        assert_eq!(8, part_one(TEST_INPUT)?.1);
        Ok(())
    }

    #[test]
    fn test_two() -> Result<()> {
        // 71274 for part 2
        assert_eq!(2286, part_two(TEST_INPUT)?.1);
        Ok(())
    }
}
