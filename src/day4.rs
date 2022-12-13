use nom::{
    Finish,
    IResult,
    character::complete,
    combinator::{all_consuming, map},
    multi::separated_list1,
    sequence::separated_pair,
};
use std::ops::RangeInclusive;
use thiserror::Error;

#[derive(Debug)]
struct ElfPair {
    left: RangeInclusive<u32>,
    right: RangeInclusive<u32>,
}

impl ElfPair {
    fn overlap_fully(&self) -> bool {
        self.left.contains(self.right.start()) && self.left.contains(self.right.end()) ||
            self.right.contains(self.left.start()) && self.right.contains(self.left.end())
    }

    fn overlap_partially(&self) -> bool {
        self.left.contains(self.right.start()) ||
            self.left.contains(self.right.end()) ||
            self.right.contains(self.left.start()) ||
            self.right.contains(self.left.end())
    }

    fn parse_range(i: &str) -> IResult<&str, RangeInclusive<u32>> {
        map(
            separated_pair(complete::u32, complete::char('-'), complete::u32),
            |(start, end)| start..=end,
        )(i)
    }

    fn parse(i: &str) -> IResult<&str, Self> {
        map(
            separated_pair(ElfPair::parse_range, complete::char(','), ElfPair::parse_range),
            |(left, right)| ElfPair { left, right },
        )(i)
    }
}

#[derive(Error, Debug)]
enum Error {
    #[error(transparent)]
    Nom(#[from] nom::error::Error<String>),
}

fn read_input(content: &str) -> Result<Vec<ElfPair>, Error> {
    let (_, elves) = all_consuming(separated_list1(complete::line_ending, ElfPair::parse))(content)
        .map_err(|e| e.to_owned())
        .finish()?;

    Ok(elves)
}

fn run_challenge1(content: &str) -> Result<u32, Error> {
    let pairs: Vec<ElfPair> = read_input(content)?;
    println!("{:?}", pairs);

    let overlaps: Vec<ElfPair> = pairs.into_iter().filter(ElfPair::overlap_fully).collect();
    println!("{:?}", overlaps);

    Ok(overlaps.len() as u32)
}

fn run_challenge2(content: &str) -> Result<u32, Error> {
    let pairs: Vec<ElfPair> = read_input(content)?;
    println!("{:?}", pairs);

    let overlaps: Vec<ElfPair> = pairs.into_iter().filter(ElfPair::overlap_partially).collect();
    println!("{:?}", overlaps);

    Ok(overlaps.len() as u32)
}


#[cfg(test)]
mod tests {
    use crate::day4::*;

    #[test]
    fn challenge1_example() -> Result<(), Error> {
        let score = run_challenge1(include_str!("data/day4_example.txt"))?;
        assert_eq!(score, 2);
        Ok(())
    }

    #[test]
    fn challenge1() -> Result<(), Error> {
        let score = run_challenge1(include_str!("data/day4_challenge.txt"))?;
        println!("{}", score);
        Ok(())
    }

    #[test]
    fn challenge2_example() -> Result<(), Error> {
        let score = run_challenge2(include_str!("data/day4_example.txt"))?;
        assert_eq!(score, 4);
        Ok(())
    }

    #[test]
    fn challenge2() -> Result<(), Error> {
        let score = run_challenge2(include_str!("data/day4_challenge.txt"))?;
        println!("{}", score);
        Ok(())
    }
}