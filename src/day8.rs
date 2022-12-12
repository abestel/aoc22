use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};
use thiserror::Error;

#[derive(Debug, Error)]
enum Error {
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error("Invalid number '{0}'")]
    InvalidNumber(char),
    #[error("Empty input")]
    EmptyInput,
    #[error("All tree lines should be of the same size")]
    InvalidTreeLines,
}

struct Tree {
    x: usize,
    y: usize,
    size: u32,
}

struct Trees {
    trees: Vec<Vec<u32>>,
    rows: usize,
    columns: usize,
}

fn is_visible(size: u32, mut trees: impl Iterator<Item=u32>) -> bool {
    trees.all(|s| s < size)
}

fn scenic_score(size: u32, trees: impl Iterator<Item=u32>) -> usize {
    let mut count = 0;
    for tree in trees {
        count += 1;
        if tree >= size {
            break;
        }
    }

    count
}

impl Trees {
    #[inline]
    fn get_trees(&self, from_x: usize, to_x: usize, from_y: usize, to_y: usize) -> impl DoubleEndedIterator<Item=u32> + '_ {
        self.trees[from_y..to_y].iter()
            .flat_map(move |line| line[from_x..to_x].iter().cloned())
    }

    fn left_trees(&self, x: usize, y: usize) -> impl DoubleEndedIterator<Item=u32> + '_ {
        self.get_trees(0, x, y, y + 1)
    }

    fn right_trees(&self, x: usize, y: usize) -> impl DoubleEndedIterator<Item=u32> + '_ {
        self.get_trees(x + 1, self.columns, y, y + 1)
    }

    fn up_trees(&self, x: usize, y: usize) -> impl DoubleEndedIterator<Item=u32> + '_ {
        self.get_trees(x, x + 1, 0, y)
    }

    fn bottom_trees(&self, x: usize, y: usize) -> impl DoubleEndedIterator<Item=u32> + '_ {
        self.get_trees(x, x + 1, y + 1, self.rows)
    }

    fn visible_trees(&self) -> usize {
        self.trees.iter().cloned().enumerate()
            .map(|(y, line)|
                line.iter().cloned().enumerate()
                    .filter(|(x, size)|
                        is_visible(*size, self.left_trees(*x, y)) ||
                            is_visible(*size, self.right_trees(*x, y)) ||
                            is_visible(*size, self.up_trees(*x, y)) ||
                            is_visible(*size, self.bottom_trees(*x, y))
                    )
                    .count()
            ).sum()
    }

    fn max_scenic_score(&self) -> Option<usize> {
        self.trees.iter().cloned().enumerate()
            .filter_map(|(y, line)|
                line.iter().cloned().enumerate()
                    .map(|(x, size)|
                        scenic_score(size, self.left_trees(x, y).rev()) *
                            scenic_score(size, self.right_trees(x, y)) *
                            scenic_score(size, self.up_trees(x, y).rev()) *
                            scenic_score(size, self.bottom_trees(x, y))
                    )
                    .max()
            ).max()
    }
}

fn read_input<P>(path: P) -> Result<Trees, Error>
    where P: AsRef<Path> {
    let file = File::open(path)?;

    let mut trees: Vec<Vec<u32>> = Vec::new();
    for line in BufReader::new(file).lines() {
        let mut current_line: Vec<u32> = Vec::new();

        for char in line?.chars() {
            current_line.push(
                char
                    .to_digit(10)
                    .ok_or(Error::InvalidNumber(char))?
            );
        }

        trees.push(current_line);
    }

    match trees.first() {
        None => Err(Error::EmptyInput),
        Some(first) => {
            let rows = trees.len();
            let expected_colums = first.len();

            if trees.iter().all(|line| line.len() == expected_colums) {
                Ok(
                    Trees {
                        trees,
                        rows,
                        columns: expected_colums,
                    }
                )
            } else {
                Err(
                    Error::InvalidTreeLines
                )
            }
        }
    }
}

fn run_challenge1<P>(path: P) -> Result<usize, Error>
    where P: AsRef<Path> {
    let trees = read_input(path)?;
    Ok(trees.visible_trees())
}

fn run_challenge2<P>(path: P) -> Result<usize, Error>
    where P: AsRef<Path> {
    let trees = read_input(path)?;
    trees.max_scenic_score().ok_or(Error::EmptyInput)
}

#[cfg(test)]
mod tests {
    use crate::day8::*;

    #[test]
    fn challenge1_example() -> Result<(), Error> {
        let sum = run_challenge1("resources/day8_example.txt")?;
        assert_eq!(sum, 21);
        Ok(())
    }

    #[test]
    fn challenge1() -> Result<(), Error> {
        let sum = run_challenge1("resources/day8_challenge.txt")?;
        dbg!(sum);
        Ok(())
    }

    #[test]
    fn challenge2_example() -> Result<(), Error> {
        let size = run_challenge2("resources/day8_example.txt")?;
        assert_eq!(size, 8);
        Ok(())
    }

    #[test]
    fn challenge2() -> Result<(), Error> {
        let sum = run_challenge2("resources/day8_challenge.txt")?;
        dbg!(sum);
        Ok(())
    }
}