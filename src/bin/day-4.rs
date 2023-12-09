use std::collections::{HashMap, HashSet, VecDeque};
use std::hash::{Hash, Hasher};

use advent_of_code_2023::parse_args;
use color_eyre::Result;

use nom::{
    bytes::complete::tag,
    character::complete::{digit1, multispace0, multispace1},
    multi::many1,
    sequence::{terminated, tuple},
    IResult,
};

static INPUT: &str = include_str!("../inputs/input-4");

#[derive(Debug, Default, Clone, PartialEq, Eq)]
struct Card {
    id: i32,
    winners: HashSet<i32>,
    mine: HashSet<i32>,
}

impl Hash for Card {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

fn memo(card: &Card, cache: &mut HashMap<Card, i32>) -> i32 {
    if let Some(result) = cache.get(card) {
        return *result;
    }

    let total = card.mine.intersection(&card.winners).count() as i32;

    cache.insert(card.clone(), total);

    total
}

fn parse_card(input: &str) -> IResult<&str, ()> {
    // 'Card 1: '
    let (input, _) = tuple((tag("Card"), multispace1, digit1, tag(":"), multispace1))(input)?;
    Ok((input, ()))
}

fn parse_numbers(input: &str) -> IResult<&str, HashSet<i32>> {
    // '41 48 83 86 17 '| 83 86  6 31 17  9 48 53
    let (input, winners) = many1(terminated(digit1, multispace0))(input)?;
    Ok((
        input,
        winners.into_iter().map(|n| n.parse().unwrap()).collect(),
    ))
}

fn parser(input: &str) -> IResult<&str, (HashSet<i32>, HashSet<i32>)> {
    // Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
    let (input, _) = parse_card(input)?;
    let (input, winners) = parse_numbers(input)?;
    let (input, _) = tuple((tag("|"), multispace1))(input)?;
    let (input, mine) = parse_numbers(input)?;
    Ok((input, (winners, mine)))
}

fn part_one(input: &str) -> IResult<&str, i32> {
    let mut total = 0;
    for line in input.lines() {
        let (_, (winners, mine)) = parser(line)?;

        let mut subtotal = 0;

        for _ in mine.intersection(&winners) {
            if subtotal == 0 {
                subtotal = 1;
            } else {
                subtotal *= 2;
            }
        }

        total += subtotal;
    }
    Ok(("", total))
}

fn part_two(input: &str) -> IResult<&str, i32> {
    let mut cards = Vec::new();
    let mut cache = HashMap::new();
    let mut copies = VecDeque::new();
    let mut total = 0;

    for (idx, line) in input.lines().enumerate() {
        let idx = idx + 1; // 1-indexed

        let (_, (winners, mine)) = parser(line)?;

        let card = Card {
            id: idx as i32,
            winners,
            mine,
        };

        cards.push(card);
    }

    copies.push_back(cards[0].id);

    let mut last_original = 1;

    while let Some(idx) = copies.pop_front() {
        if idx == -1 {
            last_original += 1;
            copies.push_back(last_original);
            continue;
        }

        let Some(card) = cards.iter().find(|c| c.id == idx) else {
            continue;
        };

        total += 1;

        let num_matches = memo(card, &mut cache);

        if num_matches == 0 {
            continue;
        }

        // enqueue the next 'num_matches' cards
        (card.id + 1..=card.id + num_matches).for_each(|idx| {
            copies.push_back(idx);
        });

        // add a placeholder to indicate the end of this card's matches
        copies.push_back(-1);
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
    static TEST_INPUT: &str = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

    #[test]
    fn test_one() -> Result<()> {
        assert_eq!(13, part_one(TEST_INPUT)?.1);
        Ok(())
    }

    #[test]
    fn test_two() -> Result<()> {
        assert_eq!(30, part_two(TEST_INPUT)?.1);
        Ok(())
    }
}
