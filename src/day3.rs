use nom::{
    Finish,
    IResult,
    character::complete,
    combinator::{all_consuming, map, map_parser, opt},
    multi::many1,
    sequence::terminated,
};
use std::{
    collections::HashSet,
    hash::Hash,
    iter::Chain,
    slice::Iter,
};
use thiserror::Error;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct Item {
    id: char,
}

impl Item {
    fn parse(i: &str) -> IResult<&str, Vec<Self>> {
        map(
            map_parser(
                complete::alpha1,
                many1(complete::anychar),
            ),
            |ids| ids.iter().cloned().map(|id| Item { id }).collect(),
        )(i)
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

impl Rucksack {
    fn parse(i: &str) -> IResult<&str, Self> {
        map(
            terminated(
                Item::parse,
                opt(complete::line_ending),
            ),
            |items| {
                let (f, s) = items.split_at(items.len() / 2);
                Rucksack { first_compartment: f.to_vec(), second_compartment: s.to_vec() }
            },
        )(i)
    }

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
    Nom(#[from] nom::error::Error<String>),
    #[error("Invalid rucksack {0:?} - {1}")]
    InvalidRuckSack(Rucksack, CommonElementError<Item>),
    #[error("Invalid rucksack group {0:?} - {1}")]
    InvalidGroup(Vec<Rucksack>, CommonElementError<Item>),
}

fn read_input(content: &str) -> Result<Vec<Rucksack>, Error> {
    let (_, rs) = all_consuming(many1(Rucksack::parse))(content)
        .map_err(|e| e.to_owned())
        .finish()?;

    Ok(rs)
}

fn run_challenge1(content: &str) -> Result<u32, Error> {
    let rucksacks: Vec<Rucksack> = read_input(content)?;

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

fn run_challenge2(content: &str) -> Result<u32, Error> {
    let rucksacks: Vec<Rucksack> = read_input(content)?;
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
        let score = run_challenge1(include_str!("data/day3_example.txt"))?;
        assert_eq!(score, 157);
        Ok(())
    }

    #[test]
    fn challenge1() -> Result<(), Error> {
        let score = run_challenge1(include_str!("data/day3_challenge.txt"))?;
        println!("{}", score);
        Ok(())
    }

    #[test]
    fn challenge2_example() -> Result<(), Error> {
        let score = run_challenge2(include_str!("data/day3_example.txt"))?;
        assert_eq!(score, 70);
        Ok(())
    }

    #[test]
    fn challenge2() -> Result<(), Error> {
        let score = run_challenge2(include_str!("data/day3_challenge.txt"))?;
        println!("{}", score);
        Ok(())
    }
}