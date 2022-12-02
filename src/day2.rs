use std::{
    fmt::Debug,
    fs::File,
    io::{BufReader, BufRead},
    path::Path,
    str::FromStr,
};
use thiserror::Error;

#[derive(Debug)]
enum Outcome {
    Win,
    Draw,
    Lost,
}

impl FromStr for Outcome {
    type Err = Error;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "X" => Ok(Outcome::Lost),
            "Y" => Ok(Outcome::Draw),
            "Z" => Ok(Outcome::Win),
            other => Err(Error::UnknownOutcome(other.to_owned()))
        }
    }
}

impl Outcome {
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

impl FromStr for Shape {
    type Err = Error;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "A" | "X" => Ok(Shape::Rock),
            "B" | "Y" => Ok(Shape::Paper),
            "C" | "Z" => Ok(Shape::Scissors),
            other => Err(Error::UnknownShape(other.to_owned()))
        }
    }
}

#[derive(Debug)]
struct Round {
    elf: Shape,
    me: Shape,
}

impl FromStr for Round {
    type Err = Error;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let mut shapes = line.split(' ').map(|value| value.parse::<Shape>());

        Ok(Round {
            elf: shapes.next().ok_or_else(|| Error::UnparsableLine(line.to_owned()))??,
            me: shapes.next().ok_or_else(|| Error::UnparsableLine(line.to_owned()))??,
        })
    }
}

impl Round {
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

impl FromStr for RoundV2 {
    type Err = Error;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let mut shapes = line.split(' ');

        Ok(RoundV2 {
            elf: shapes.next().ok_or_else(|| Error::UnparsableLine(line.to_owned()))?.parse()?,
            me: shapes.next().ok_or_else(|| Error::UnparsableLine(line.to_owned()))?.parse()?,
        })
    }
}

impl RoundV2 {
    fn score(&self) -> u32 {
        let shape_score = self.elf.deduce_from_outcome(&self.me).score();
        let outcome_score = self.me.score();
        shape_score + outcome_score
    }
}

#[derive(Error, Debug)]
enum Error {
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error("Unknown shape '{0}'")]
    UnknownShape(String),
    #[error("Unknown outcome '{0}'")]
    UnknownOutcome(String),
    #[error("Unparsable line '{0}'")]
    UnparsableLine(String),
}

fn read_input<P, R>(path: P) -> Result<Vec<R>, Error>
    where P: AsRef<Path>,
          R: FromStr<Err=Error> {
    let file = File::open(path)?;
    let lines = BufReader::new(file).lines();

    let mut rounds: Vec<R> = Vec::new();

    for line in lines {
        rounds.push(line?.parse::<R>()?);
    }

    Ok(rounds)
}

fn run_challenge1<P>(path: P) -> Result<u32, anyhow::Error>
    where P: AsRef<Path> {
    let rounds: Vec<Round> = read_input(path)?;
    Ok(rounds.iter().map(Round::score).sum())
}

fn run_challenge2<P>(path: P) -> Result<u32, anyhow::Error>
    where P: AsRef<Path> {
    let rounds: Vec<RoundV2> = read_input(path)?;
    Ok(rounds.iter().map(RoundV2::score).sum())
}

#[cfg(test)]
mod tests {
    use crate::day2::*;

    #[test]
    fn challenge1_example() -> Result<(), anyhow::Error> {
        let score = run_challenge1("resources/day2_example.txt")?;
        assert_eq!(score, 15);
        Ok(())
    }

    #[test]
    fn challenge1() -> Result<(), anyhow::Error> {
        let score = run_challenge1("resources/day2_challenge.txt")?;
        println!("{}", score);
        Ok(())
    }

    #[test]
    fn challenge2_example() -> Result<(), anyhow::Error> {
        let score = run_challenge2("resources/day2_example.txt")?;
        assert_eq!(score, 12);
        Ok(())
    }

    #[test]
    fn challenge2() -> Result<(), anyhow::Error> {
        let score = run_challenge2("resources/day2_challenge.txt")?;
        println!("{}", score);
        Ok(())
    }
}