use advent_of_code_2022::parse_args;
use color_eyre::Result;

static INPUT: &str = include_str!("../inputs/input-DAY_NUM");

fn part_one(input: &str) -> Result<i32> {
    Ok(0)
}

fn part_two(input: &str) -> Result<i32> {
    Ok(0)
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
    static TEST_INPUT: &str = "";

    #[test]
    fn test_one() -> Result<()> {
        assert_eq!(0, part_one(TEST_INPUT)?);
        Ok(())
    }

    #[test]
    fn test_two() -> Result<()> {
        assert_eq!(0, part_two(TEST_INPUT)?);
        Ok(())
    }
}
