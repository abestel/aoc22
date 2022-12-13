use nom::{
    Finish,
    IResult,
    branch::alt,
    bytes::complete::tag,
    character::complete,
    combinator::{all_consuming, map, opt, value},
    multi::many1,
    sequence::separated_pair,
};
use nom::sequence::terminated;
use thiserror::Error;

#[derive(Clone, Debug)]
enum Outcome {
    Win,
    Draw,
    Lost,
}

impl Outcome {
    fn parse(i: &str) -> IResult<&str, Self> {
        alt((
            value(Outcome::Lost, tag("X")),
            value(Outcome::Draw, tag("Y")),
            value(Outcome::Win, tag("Z")),
        ))(i)
    }

    fn score(&self) -> u32 {
        match self {
            Outcome::Win => 6_u32,
            Outcome::Draw => 3_u32,
            Outcome::Lost => 0_u32,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
enum Shape {
    Rock,
    Paper,
    Scissors,
}

impl Shape {
    fn parse(i: &str) -> IResult<&str, Self> {
        alt((
            value(Shape::Rock, tag("X")),
            value(Shape::Rock, tag("A")),
            value(Shape::Paper, tag("Y")),
            value(Shape::Paper, tag("B")),
            value(Shape::Scissors, tag("Z")),
            value(Shape::Scissors, tag("C")),
        ))(i)
    }

    fn against(&self, other: &Shape) -> Outcome {
        match (self, other) {
            (first, second) if first == second => Outcome::Draw,
            (Shape::Rock, Shape::Scissors) => Outcome::Win,
            (Shape::Scissors, Shape::Paper) => Outcome::Win,
            (Shape::Paper, Shape::Rock) => Outcome::Win,
            _ => Outcome::Lost
        }
    }

    fn score(&self) -> u32 {
        match self {
            Shape::Rock => 1_u32,
            Shape::Paper => 2_u32,
            Shape::Scissors => 3_u32,
        }
    }

    fn deduce_from_outcome(&self, outcome: &Outcome) -> Shape {
        match (self, outcome) {
            (shape, Outcome::Draw) => shape.to_owned(),
            (Shape::Rock, Outcome::Lost) => Shape::Scissors,
            (Shape::Rock, Outcome::Win) => Shape::Paper,
            (Shape::Paper, Outcome::Lost) => Shape::Rock,
            (Shape::Paper, Outcome::Win) => Shape::Scissors,
            (Shape::Scissors, Outcome::Lost) => Shape::Paper,
            (Shape::Scissors, Outcome::Win) => Shape::Rock,
        }
    }
}

#[derive(Debug)]
struct Round {
    elf: Shape,
    me: Shape,
}

impl Round {
    fn parse(i: &str) -> IResult<&str, Self> {
        terminated(
            map(
                separated_pair(Shape::parse, complete::space1, Shape::parse),
                |(elf, me)| Round { elf, me },
            ),
            opt(complete::line_ending),
        )(i)
    }

    fn score(&self) -> u32 {
        let shape_score = self.me.score();
        let outcome_score = self.me.against(&self.elf).score();
        shape_score + outcome_score
    }
}

#[derive(Debug)]
struct RoundV2 {
    elf: Shape,
    me: Outcome,
}

impl RoundV2 {
    fn parse(i: &str) -> IResult<&str, Self> {
        terminated(
            map(
                separated_pair(Shape::parse, complete::space1, Outcome::parse),
                |(elf, me)| RoundV2 { elf, me },
            ),
            opt(complete::line_ending),
        )(i)
    }

    fn score(&self) -> u32 {
        let shape_score = self.elf.deduce_from_outcome(&self.me).score();
        let outcome_score = self.me.score();
        shape_score + outcome_score
    }
}

#[derive(Error, Debug)]
enum Error {
    #[error(transparent)]
    Nom(#[from] nom::error::Error<String>),
}

fn run_challenge1(content: &str) -> Result<u32, Error> {
    let (_, rounds) = all_consuming(many1(Round::parse))(content)
        .map_err(|e| e.to_owned())
        .finish()?;
    Ok(rounds.iter().map(Round::score).sum())
}

fn run_challenge2(content: &str) -> Result<u32, anyhow::Error> {
    let (_, rounds) = all_consuming(many1(RoundV2::parse))(content)
        .map_err(|e| e.to_owned())
        .finish()?;
    Ok(rounds.iter().map(RoundV2::score).sum())
}

#[cfg(test)]
mod tests {
    use crate::day2::*;

    #[test]
    fn challenge1_example() -> Result<(), anyhow::Error> {
        let score = run_challenge1(include_str!("data/day2_example.txt"))?;
        assert_eq!(score, 15);
        Ok(())
    }

    #[test]
    fn challenge1() -> Result<(), anyhow::Error> {
        let score = run_challenge1(include_str!("data/day2_challenge.txt"))?;
        println!("{}", score);
        Ok(())
    }

    #[test]
    fn challenge2_example() -> Result<(), anyhow::Error> {
        let score = run_challenge2(include_str!("data/day2_example.txt"))?;
        assert_eq!(score, 12);
        Ok(())
    }

    #[test]
    fn challenge2() -> Result<(), anyhow::Error> {
        let score = run_challenge2(include_str!("data/day2_challenge.txt"))?;
        println!("{}", score);
        Ok(())
    }
}