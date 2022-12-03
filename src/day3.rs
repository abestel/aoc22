use std::{
    collections::HashSet,
    fmt::Debug,
    fs::File,
    io::{BufReader, BufRead},
    iter::Chain,
    path::Path,
    slice::Iter,
    str::FromStr,
};
use std::hash::Hash;
use thiserror::Error;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct Item {
    id: char,
}

impl TryFrom<char> for Item {
    type Error = Error;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        if ('a'..='z').contains(&value) || ('A'..='Z').contains(&value) {
            Ok(Item { id: value })
        } else {
            Err(Error::UnparsableItem(value))
        }
    }
}

impl Item {
    fn from_str(s: &str) -> Result<Vec<Item>, Error> {
        s.chars().map(Item::try_from).collect()
    }

    fn priority(&self) -> u32 {
        if self.id.is_lowercase() {
            (self.id as u8 - b'a') as u32 + 1
        } else {
            (self.id as u8 - b'A') as u32 + 27
        }
    }
}

#[derive(Clone, Debug)]
struct Rucksack {
    first_compartment: Vec<Item>,
    second_compartment: Vec<Item>,
}

impl FromStr for Rucksack {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() % 2 == 0 {
            let (first, second) = s.split_at(s.len() / 2);
            Ok(Rucksack {
                first_compartment: Item::from_str(first)?,
                second_compartment: Item::from_str(second)?,
            })
        } else {
            Err(Error::UnbalancedRuckSack(s.to_owned()))
        }
    }
}

impl Rucksack {
    fn common(&self) -> Result<&Item, Error> {
        let Rucksack { first_compartment, second_compartment } = self;
        common_element(first_compartment, vec![second_compartment])
            .map_err(|x| Error::InvalidRuckSack(self.clone(), x))
    }

    fn elements(&self) -> Chain<Iter<Item>, Iter<Item>> {
        self.first_compartment.iter().chain(self.second_compartment.iter())
    }
}

fn intersect<'a, I, T>(head: I, tail: Vec<I>) -> HashSet<&'a T>
    where I: IntoIterator<Item=&'a T>,
          T: Eq + Hash + 'a {
    let mut intersection: HashSet<&T> = HashSet::from_iter(head.into_iter());

    for item in tail {
        intersection = HashSet::from_iter(item.into_iter())
            .intersection(&intersection)
            .cloned()
            .collect();
    }

    intersection
}

fn common_element<'a, I, T>(head: I, tail: Vec<I>) -> Result<&'a T, CommonElementError<T>>
    where I: IntoIterator<Item=&'a T>,
          T: Clone + Eq + Hash + 'a {
    let common: HashSet<&T> = intersect(head, tail);

    if common.len() > 1 {
        Err(CommonElementError::TooManyCommonItems(common.into_iter().cloned().collect()))
    } else {
        match common.iter().next() {
            None => Err(CommonElementError::NoCommonItem),
            Some(item) => Ok(*item)
        }
    }
}

#[derive(Error, Debug)]
enum CommonElementError<I> {
    #[error("No common item found")]
    NoCommonItem,
    #[error("Too many common items founds {0:?}")]
    TooManyCommonItems(Vec<I>),
}

#[derive(Error, Debug)]
enum Error {
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error("Unparsable Item '{0}")]
    UnparsableItem(char),
    #[error("Unbalanced rucksack '{0}'")]
    UnbalancedRuckSack(String),
    #[error("Invalid rucksack {0:?} - {1}")]
    InvalidRuckSack(Rucksack, CommonElementError<Item>),
    #[error("Invalid rucksack group {0:?} - {1}")]
    InvalidGroup(Vec<Rucksack>, CommonElementError<Item>),
}

fn read_input<P>(path: P) -> Result<Vec<Rucksack>, Error>
    where P: AsRef<Path> {
    let file = File::open(path)?;
    let lines = BufReader::new(file).lines();

    let mut rucksacks: Vec<Rucksack> = Vec::new();

    for line in lines {
        rucksacks.push(line?.parse::<Rucksack>()?);
    }

    Ok(rucksacks)
}

fn run_challenge1<P>(path: P) -> Result<u32, Error>
    where P: AsRef<Path> {
    let rucksacks: Vec<Rucksack> = read_input(path)?;

    let common = rucksacks
        .iter()
        .map(Rucksack::common)
        .collect::<Result<Vec<&Item>, Error>>()?;

    Ok(
        common
            .iter()
            .cloned()
            .map(Item::priority)
            .sum()
    )
}

fn run_challenge2<P>(path: P) -> Result<u32, Error>
    where P: AsRef<Path> {
    let rucksacks: Vec<Rucksack> = read_input(path)?;
    let groups = rucksacks
        .chunks_exact(3)
        .map(|group| {
            let (head, tail) = group.split_first().unwrap();
            common_element(head.elements(), tail.iter().map(Rucksack::elements).collect())
                .map_err(|x| Error::InvalidGroup(group.to_vec(), x))
        })
        .collect::<Result<Vec<&Item>, Error>>()?;

    Ok(
        groups
            .iter()
            .cloned()
            .map(Item::priority)
            .sum()
    )
}


#[cfg(test)]
mod tests {
    use crate::day3::*;

    #[test]
    fn challenge1_example() -> Result<(), Error> {
        let score = run_challenge1("resources/day3_example.txt")?;
        assert_eq!(score, 157);
        Ok(())
    }

    #[test]
    fn challenge1() -> Result<(), Error> {
        let score = run_challenge1("resources/day3_challenge.txt")?;
        println!("{}", score);
        Ok(())
    }

    #[test]
    fn challenge2_example() -> Result<(), Error> {
        let score = run_challenge2("resources/day3_example.txt")?;
        assert_eq!(score, 70);
        Ok(())
    }

    #[test]
    fn challenge2() -> Result<(), Error> {
        let score = run_challenge2("resources/day3_challenge.txt")?;
        println!("{}", score);
        Ok(())
    }
}