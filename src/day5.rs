use lazy_static::lazy_static;
use regex::Regex;
use std::{
    fmt::{Debug, Display, Formatter},
    str::{self, FromStr},
};
use thiserror::Error;

struct Stacks {
    stacks: Vec<Vec<char>>,
}

impl Display for Stacks {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if let Some(max_height) = self.stacks.iter().map(Vec::len).max() {
            for i in (0..max_height).rev() {
                let crates_at_index = self.stacks.iter().map(|stack| stack.get(i));
                for (index, maybe_crate) in crates_at_index.enumerate() {
                    match maybe_crate {
                        None => f.write_str("   ")?,
                        Some(c) => f.write_fmt(format_args!("[{}]", c))?
                    };

                    if index != self.stacks.len() - 1 {
                        f.write_str(" ")?;
                    }
                }

                f.write_str("\n")?;
            }

            for i in 0..self.stacks.len() {
                f.write_fmt(format_args!(" {}  ", i + 1))?;
            }

            Ok(())
        } else {
            f.write_str("<empty>\n")
        }
    }
}

impl Stacks {
    fn new() -> Stacks {
        Stacks {
            stacks: Vec::new(),
        }
    }

    fn accept(&self, action: &CraneAction) -> Result<Self, Error> {
        match *action {
            CraneAction::Move { number_crates, from_stack, to_stack } => {
                if from_stack > self.stacks.len() {
                    Err(Error::InvalidStackReference(from_stack, action.clone()))
                } else if to_stack > self.stacks.len() {
                    Err(Error::InvalidStackReference(to_stack, action.clone()))
                } else if self.stacks[from_stack - 1].len() < number_crates {
                    Err(Error::ImpossibleToApplyAction(self.stacks[from_stack - 1].clone(), action.clone()))
                } else {
                    let mut stacks = self.stacks.clone();

                    let from_index = stacks[from_stack - 1].len() - number_crates;
                    let to_index = stacks[from_stack - 1].len();

                    // Add to
                    let to_move: Vec<char> = stacks[from_stack - 1].as_slice()[from_index..to_index].to_vec();
                    for item in to_move.iter().rev() {
                        stacks[to_stack - 1].push(*item);
                    }

                    // Remove in the from stack
                    stacks[from_stack - 1].splice(from_index..to_index, vec![]);

                    Ok(Stacks { stacks })
                }
            }
        }
    }

    fn accept_v2(&self, action: &CraneAction) -> Result<Self, Error> {
        match *action {
            CraneAction::Move { number_crates, from_stack, to_stack } => {
                if from_stack > self.stacks.len() {
                    Err(Error::InvalidStackReference(from_stack, action.clone()))
                } else if to_stack > self.stacks.len() {
                    Err(Error::InvalidStackReference(to_stack, action.clone()))
                } else if self.stacks[from_stack - 1].len() < number_crates {
                    Err(Error::ImpossibleToApplyAction(self.stacks[from_stack - 1].clone(), action.clone()))
                } else {
                    let mut stacks = self.stacks.clone();

                    let from_index = stacks[from_stack - 1].len() - number_crates;
                    let to_index = stacks[from_stack - 1].len();

                    // Add to
                    let to_move: Vec<char> = stacks[from_stack - 1].as_slice()[from_index..to_index].to_vec();
                    for item in to_move.iter() {
                        stacks[to_stack - 1].push(*item);
                    }

                    // Remove in the from stack
                    stacks[from_stack - 1].splice(from_index..to_index, vec![]);

                    Ok(Stacks { stacks })
                }
            }
        }
    }
}

impl TryFrom<Vec<StackLine>> for Stacks {
    type Error = Error;

    fn try_from(lines: Vec<StackLine>) -> Result<Self, Self::Error> {
        if lines.is_empty() {
            Ok(Stacks::new())
        } else {
            let stacks_number = lines[0].crates.len();

            if lines.iter().all(|line| line.crates.len() == stacks_number) {
                let mut stacks: Vec<Vec<char>> = Vec::with_capacity(stacks_number);
                for _ in 0..stacks_number {
                    stacks.push(Vec::new());
                }

                for line in lines {
                    for (index, c) in line.crates.iter().enumerate() {
                        if let Some(c) = c {
                            stacks[index].insert(0, *c);
                        }
                    }
                }

                Ok(Stacks { stacks })
            } else {
                Err(Error::InvalidStacks(lines))
            }
        }
    }
}

#[derive(Debug)]
struct StackLine {
    crates: Vec<Option<char>>,
}

impl FromStr for StackLine {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(
            StackLine {
                crates: s
                    .as_bytes()
                    .chunks(4)
                    .map(|c| {
                        if c.iter().all(|x| *x == b' ') {
                            Ok(None)
                        } else if c.len() >= 3 && c[0] == b'[' && c[2] == b']' {
                            Ok(Some(c[1] as char))
                        } else {
                            Err(
                                Error::InvalidCrate(
                                    str::from_utf8(c).unwrap().to_string()
                                )
                            )
                        }
                    })
                    .collect::<Result<Vec<_>, Error>>()?
            }
        )
    }
}

#[derive(Debug, Clone)]
enum CraneAction {
    Move {
        number_crates: usize,
        from_stack: usize,
        to_stack: usize,
    }
}

impl FromStr for CraneAction {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref MOVE_RE: Regex = Regex::new("move (\\d+) from (\\d+) to (\\d+)").unwrap();
        }

        if let Some(capture) = MOVE_RE.captures(s) {
            if capture.len() == 4 {
                Ok(
                    CraneAction::Move {
                        number_crates: capture[1].parse()?,
                        from_stack: capture[2].parse()?,
                        to_stack: capture[3].parse()?,
                    }
                )
            } else {
                Err(
                    Error::InvalidMove(s.to_string())
                )
            }
        } else {
            Err(
                Error::InvalidMove(s.to_string())
            )
        }
    }
}

#[derive(Error, Debug)]
enum Error {
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error("Invalid crate '{0}'")]
    InvalidCrate(String),
    #[error("Invalid stacks '{0:?}'")]
    InvalidStacks(Vec<StackLine>),
    #[error("Invalid move '{0:?}'")]
    InvalidMove(String),
    #[error(transparent)]
    ParseInt(#[from] std::num::ParseIntError),
    #[error("Invalid stack '{0}' referenced in action '{1:?}'")]
    InvalidStackReference(usize, CraneAction),
    #[error("Impossible to apply action '{1:?}' on stack '{0:?}'")]
    ImpossibleToApplyAction(Vec<char>, CraneAction),
}

enum ReadAction {
    ReadStackLines,
    Skip(u16, Box<ReadAction>),
    ReadCraneActions,
}

fn read_input(content: &str) -> Result<(Stacks, Vec<CraneAction>), Error> {
    let mut stack_lines: Vec<StackLine> = Vec::new();
    let mut actions: Vec<CraneAction> = Vec::new();

    let mut read = ReadAction::ReadStackLines;

    for line in content.lines() {
        match read {
            ReadAction::ReadStackLines => {
                if line.starts_with('[') || line.starts_with("    ") {
                    stack_lines.push(line.parse()?);
                } else {
                    read = ReadAction::Skip(1_u16, Box::new(ReadAction::ReadCraneActions));
                }
            }

            ReadAction::Skip(lines, next) => {
                if lines == 1 {
                    read = *next;
                } else {
                    read = ReadAction::Skip(lines - 1, next);
                }
            }

            ReadAction::ReadCraneActions => {
                actions.push(line.parse()?);
            }
        }
    }

    Ok(
        (
            Stacks::try_from(stack_lines)?,
            actions
        )
    )
}

fn run_challenge1(content: &str) -> Result<String, Error> {
    let (mut stacks, actions) = read_input(content)?;

    println!("Initial state:\n{}\n", stacks);

    for (index, action) in actions.iter().enumerate() {
        stacks = stacks.accept(action)?;
        println!("Step {} - {:?}:\n{}\n", index + 1, action, stacks);
    };

    Ok(
        stacks.stacks
            .iter()
            .flat_map(|x| x.last())
            .cloned()
            .collect()
    )
}

fn run_challenge2(content: &str) -> Result<String, Error> {
    let (mut stacks, actions) = read_input(content)?;

    println!("Initial state:\n{}\n", stacks);

    for (index, action) in actions.iter().enumerate() {
        stacks = stacks.accept_v2(action)?;
        println!("Step {} - {:?}:\n{}\n", index + 1, action, stacks);
    };

    Ok(
        stacks.stacks
            .iter()
            .flat_map(|x| x.last())
            .cloned()
            .collect()
    )
}


#[cfg(test)]
mod tests {
    use crate::day5::*;

    #[test]
    fn challenge1_example() -> Result<(), Error> {
        let result = run_challenge1(include_str!("data/day5_example.txt"))?;
        assert_eq!(result, "CMZ");
        Ok(())
    }

    #[test]
    fn challenge1() -> Result<(), Error> {
        let result = run_challenge1(include_str!("data/day5_challenge.txt"))?;
        println!("{}", result);
        Ok(())
    }

    #[test]
    fn challenge2_example() -> Result<(), Error> {
        let result = run_challenge2(include_str!("data/day5_example.txt"))?;
        assert_eq!(result, "MCD");
        Ok(())
    }

    #[test]
    fn challenge2() -> Result<(), Error> {
        let result = run_challenge2(include_str!("data/day5_challenge.txt"))?;
        println!("{}", result);
        Ok(())
    }
}