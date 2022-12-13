use nom::{
    Finish,
    IResult,
    character::complete,
    combinator::{all_consuming, opt},
    multi::many1,
    sequence::terminated,
};
use thiserror::Error;

fn parse_elf(i: &str) -> IResult<&str, Vec<u64>> {
    terminated(
        many1(
            terminated(complete::u64, complete::line_ending),
        ),
        opt(complete::line_ending),
    )(i)
}

fn read_input(content: &str) -> Result<Vec<Vec<u64>>, Error> {
    let (_, elves) = all_consuming(many1(parse_elf))(content)
        .map_err(|e| e.to_owned())
        .finish()?;

    Ok(elves)
}

fn compute_calories(elves: Vec<Vec<u64>>) -> Vec<u64> {
    elves
        .into_iter()
        .map(|elf| elf.into_iter().sum::<u64>())
        .collect()
}

fn max_calories(elves_calories: Vec<u64>) -> Option<u64> {
    elves_calories.into_iter().max()
}

fn run_challenge1(content: &str) -> Result<u64, Error> {
    let elves = read_input(content)?;
    let elves = compute_calories(elves);
    Ok(max_calories(elves).unwrap_or_default())
}

fn run_challenge2(content: &str) -> Result<u64, Error> {
    let elves = read_input(content)?;
    let mut elves = compute_calories(elves);
    elves.sort();

    Ok(elves.iter().rev().take(3).sum())
}

#[derive(Debug, Error)]
enum Error {
    #[error(transparent)]
    Nom(#[from] nom::error::Error<String>),
}

#[cfg(test)]
mod tests {
    use crate::day1::*;

    #[test]
    fn challenge1_example() -> Result<(), Error> {
        let max = run_challenge1(include_str!("data/day1_example.txt"))?;
        assert_eq!(max, 24000);
        Ok(())
    }

    #[test]
    fn challenge1() -> Result<(), Error> {
        let max = run_challenge1(include_str!("data/day1_challenge.txt"))?;
        println!("{}", max);
        Ok(())
    }

    #[test]
    fn challenge2_example() -> Result<(), Error> {
        let top3 = run_challenge2(include_str!("data/day1_example.txt"))?;
        assert_eq!(top3, 45000);
        Ok(())
    }

    #[test]
    fn challenge2() -> Result<(), Error> {
        let top3 = run_challenge2(include_str!("data/day1_challenge.txt"))?;
        println!("{}", top3);
        Ok(())
    }
}