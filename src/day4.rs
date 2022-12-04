use std::{
    fmt::Debug,
    fs::File,
    io::{BufReader, BufRead},
    ops::RangeInclusive,
    path::Path,
    str::FromStr,
};
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

    fn read_range_bound(bound: &str) -> Result<u32, Error> {
        bound.parse()
            .map_err(|e| Error::InvalidRangeBound(e, bound.to_owned()))
    }

    fn read_range(range: &str) -> Result<RangeInclusive<u32>, Error> {
        let (first, second) = range.split_once('-')
            .ok_or_else(|| Error::NoDelimiterRange(range.to_owned()))?;

        Ok(
            RangeInclusive::new(
                ElfPair::read_range_bound(first)?,
                ElfPair::read_range_bound(second)?,
            )
        )
    }
}

impl FromStr for ElfPair {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (first, second) = s.split_once(',')
            .ok_or_else(|| Error::UnparsableLine(s.to_owned()))?;

        Ok(
            ElfPair {
                left: ElfPair::read_range(first)?,
                right: ElfPair::read_range(second)?,
            }
        )
    }
}

#[derive(Error, Debug)]
enum Error {
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error("Unparsable line '{0}'")]
    UnparsableLine(String),
    #[error("Unparsable range '{0}'")]
    UnparsableRange(String),
    #[error("Range '{0}' does not contain two bounds separated by a '-'")]
    NoDelimiterRange(String),
    #[error("Unparsable range '{0}'")]
    InvalidRangeBound(std::num::ParseIntError, String),
}

fn read_input<P>(path: P) -> Result<Vec<ElfPair>, Error>
    where P: AsRef<Path> {
    let file = File::open(path)?;
    let lines = BufReader::new(file).lines();

    let mut pairs: Vec<ElfPair> = Vec::new();

    for line in lines {
        pairs.push(line?.parse::<ElfPair>()?);
    }

    Ok(pairs)
}

fn run_challenge1<P>(path: P) -> Result<u32, Error>
    where P: AsRef<Path> {
    let pairs: Vec<ElfPair> = read_input(path)?;
    println!("{:?}", pairs);

    let overlaps: Vec<ElfPair> = pairs.into_iter().filter(ElfPair::overlap_fully).collect();
    println!("{:?}", overlaps);

    Ok(overlaps.len() as u32)
}

fn run_challenge2<P>(path: P) -> Result<u32, Error>
    where P: AsRef<Path> {
    let pairs: Vec<ElfPair> = read_input(path)?;
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
        let score = run_challenge1("resources/day4_example.txt")?;
        assert_eq!(score, 2);
        Ok(())
    }

    #[test]
    fn challenge1() -> Result<(), Error> {
        let score = run_challenge1("resources/day4_challenge.txt")?;
        println!("{}", score);
        Ok(())
    }

    #[test]
    fn challenge2_example() -> Result<(), Error> {
        let score = run_challenge2("resources/day4_example.txt")?;
        assert_eq!(score, 4);
        Ok(())
    }

    #[test]
    fn challenge2() -> Result<(), Error> {
        let score = run_challenge2("resources/day4_challenge.txt")?;
        println!("{}", score);
        Ok(())
    }
}