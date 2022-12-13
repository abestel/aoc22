use nom::{
    Finish,
    IResult,
    branch::alt,
    bytes::complete::tag,
    character::complete,
    combinator::{all_consuming, map, value},
    sequence::separated_pair,
};
use std::collections::HashSet;
use thiserror::Error;

#[derive(Clone, Copy, Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn parse(i: &str) -> IResult<&str, Self> {
        alt((
            value(Self::Up, tag("U")),
            value(Self::Down, tag("D")),
            value(Self::Left, tag("L")),
            value(Self::Right, tag("R")),
        ))(i)
    }

    fn as_pos(&self) -> Pos {
        match self {
            Direction::Up => Pos { x: 0, y: 1 },
            Direction::Down => Pos { x: 0, y: -1 },
            Direction::Left => Pos { x: -1, y: 0 },
            Direction::Right => Pos { x: 1, y: 0 },
        }
    }
}

#[derive(Debug)]
struct Command {
    direction: Direction,
    delta: u32,
}

impl Command {
    fn parse(i: &str) -> IResult<&str, Self> {
        map(
            separated_pair(Direction::parse, complete::space1, complete::u32),
            |(direction, delta)| Self { direction, delta },
        )(i)
    }

    fn iterator(&self) -> impl Iterator<Item=Direction> {
        std::iter::repeat(self.direction).take(self.delta as usize)
    }
}

fn read_input(content: &str) -> Result<Vec<Command>, Error> {
    let mut commands = Vec::new();
    for line in content.lines() {
        let (_, command) = all_consuming(Command::parse)(line)
            .map_err(|e| e.to_owned())
            .finish()?;

        commands.push(command);
    }

    Ok(commands)
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct Pos {
    x: i32,
    y: i32,
}


impl std::ops::Add for Pos {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Pos {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl std::ops::AddAssign for Pos {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs
    }
}

impl std::ops::Sub for Pos {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Pos {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl std::ops::SubAssign for Pos {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs
    }
}

struct Grid {
    knots: Vec<Pos>,
}

impl Grid {
    fn new(knots: usize) -> Self {
        Self { knots: std::iter::repeat(Pos { x: 0, y: 0 }).take(knots).collect() }
    }

    fn move_head(&mut self, direction: Direction) {
        self.knots[0] += direction.as_pos();

        for i in 1..self.knots.len() {
            let diff = self.knots[i - 1] - self.knots[i];
            let delta_tail = match (diff.x, diff.y) {
                (0, 0) |
                (0, -1) | (0, 1) | (-1, 0) | (1, 0) |
                (-1, 1) | (-1, -1) | (1, -1) | (1, 1) => Pos { x: 0, y: 0 },

                (0, 2) => Pos { x: 0, y: 1 },
                (0, -2) => Pos { x: 0, y: -1 },
                (2, 0) => Pos { x: 1, y: 0 },
                (-2, 0) => Pos { x: -1, y: 0 },

                (2, 1) | (1, 2) | (2, 2) => Pos { x: 1, y: 1 },
                (2, -1) | (1, -2) | (2, -2) => Pos { x: 1, y: -1 },
                (-2, 1) | (-1, 2) | (-2, 2) => Pos { x: -1, y: 1 },
                (-2, -1) | (-1, -2) | (-2, -2) => Pos { x: -1, y: -1 },

                _ => panic!("There is a bug somewhere, unhandled delta {:?}", diff)
            };

            self.knots[i] += delta_tail;
        }
    }
}

fn run_challenge1(content: &str) -> Result<HashSet<Pos>, Error> {
    let commands = read_input(content)?;

    let grid_size = 2;
    let mut grid = Grid::new(grid_size);
    let mut tail_pos = HashSet::new();

    for direction in commands.iter().flat_map(Command::iterator) {
        grid.move_head(direction);
        tail_pos.insert(grid.knots[grid_size - 1]);
    }

    Ok(tail_pos)
}

fn run_challenge2(content: &str) -> Result<HashSet<Pos>, Error> {
    let commands = read_input(content)?;

    let grid_size = 10;
    let mut grid = Grid::new(grid_size);
    let mut tail_pos = HashSet::new();

    for direction in commands.iter().flat_map(Command::iterator) {
        grid.move_head(direction);
        tail_pos.insert(grid.knots[grid_size - 1]);
    }

    Ok(tail_pos)
}

#[derive(Error, Debug)]
enum Error {
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    Nom(#[from] nom::error::Error<String>),
}

#[cfg(test)]
mod tests {
    use crate::day9::*;

    #[test]
    fn challenge1_example() -> Result<(), Error> {
        let result = run_challenge1(include_str!("data/day9_example.txt"))?;
        assert_eq!(result.len(), 13);
        Ok(())
    }

    #[test]
    fn challenge1() -> Result<(), Error> {
        let result = run_challenge1(include_str!("data/day9_challenge.txt"))?;
        dbg!(result.len());
        Ok(())
    }

    #[test]
    fn challenge2_example() -> Result<(), Error> {
        let result = run_challenge2(include_str!("data/day9_example.txt"))?;
        assert_eq!(result.len(), 1);
        Ok(())
    }

    #[test]
    fn challenge2_example2() -> Result<(), Error> {
        let result = run_challenge2(include_str!("data/day9_example2.txt"))?;
        assert_eq!(result.len(), 36);
        Ok(())
    }

    #[test]
    fn challenge2() -> Result<(), Error> {
        let result = run_challenge2(include_str!("data/day9_challenge.txt"))?;
        dbg!(result.len());
        Ok(())
    }
}