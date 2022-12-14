use std::collections::{HashMap, HashSet};
use nom::{
    Finish,
    IResult,
    branch::alt,
    character::complete,
    combinator::{all_consuming, map, value},
    multi::{many1, separated_list1},
};
use thiserror::Error;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct Pos {
    x: usize,
    y: usize,
}

#[derive(Clone, Copy, Debug)]
enum Cell {
    Start,
    End,
    Height(u8),
}

impl Cell {
    const MIN_HEIGHT: u8 = 0;
    const MAX_HEIGHT: u8 = b'z' - b'a';

    fn is_start(&self) -> bool {
        matches!(self, Cell::Start)
    }

    fn is_end(&self) -> bool {
        matches!(self, Cell::End)
    }

    fn parse(i: &str) -> IResult<&str, Self> {
        let start_parser = value(Cell::Start, complete::char('S'));
        let end_parser = value(Cell::End, complete::char('E'));
        let height_parser = map(complete::satisfy(|c| ('a'..='z').contains(&c)), |c| Cell::Height(c as u8 - b'a'));

        alt((
            start_parser,
            end_parser,
            height_parser,
        ))(i)
    }

    fn height(self) -> u8 {
        match self {
            Cell::Start => Self::MIN_HEIGHT,
            Cell::End => Self::MAX_HEIGHT,
            Cell::Height(height) => height,
        }
    }
}

#[derive(Clone, Debug)]
struct Topology {
    cells: Vec<Vec<Cell>>,
    rows: usize,
    columns: usize,
}

impl Topology {
    const NEIGHBOURS_DELTAS: [(isize, isize); 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];

    fn parse(i: &str) -> Result<Self, Error> {
        let (_, cells) = all_consuming(
            separated_list1(
                complete::line_ending,
                many1(Cell::parse),
            )
        )(i)
            .map_err(|e| e.to_owned())
            .finish()?;

        match cells.first() {
            None => Err(Error::EmptyInput),
            Some(first) => {
                let rows = cells.len();
                let expected_colums = first.len();

                if cells.iter().all(|line| line.len() == expected_colums) {
                    Ok(
                        Topology {
                            cells,
                            rows,
                            columns: expected_colums,
                        }
                    )
                } else {
                    Err(
                        Error::InvalidLineSize
                    )
                }
            }
        }
    }

    fn at(&self, pos: &Pos) -> Cell {
        self.cells[pos.y][pos.x]
    }

    fn find(&self, predicate: fn(&Cell) -> bool) -> Option<Pos> {
        self.cells
            .iter()
            .enumerate()
            .find_map(|(y, cells)|
                cells.iter()
                    .enumerate()
                    .find_map(|(x, cell)|
                        if predicate(cell) {
                            Some(Pos { x, y })
                        } else {
                            None
                        }
                    )
            )
    }

    fn neighbours(&self, pos: Pos) -> impl Iterator<Item=(Pos, Cell)> + '_ {
        Self::NEIGHBOURS_DELTAS
            .into_iter()
            .map(move |(delta_x, delta_y)| (delta_x + (pos.x as isize), delta_y + (pos.y as isize)))
            .filter(|(x, y)| 0 <= *x && *x < (self.columns as isize) && 0 <= *y && *y < (self.rows as isize))
            .map(|(x, y)| Pos { x: x as usize, y: y as usize })
            .map(|pos| (pos, self.at(&pos)))
    }
}

fn walk(
    topology: Topology,
    start_filter: fn(&Cell) -> bool,
    neighbour_filter: fn(&Cell, &Cell) -> bool,
    termination: fn(&Cell) -> bool,
) -> Result<Vec<Pos>, Error> {
    let start = topology.find(start_filter);

    match start {
        None => Err(Error::NoStartFound)?,
        Some(start) => {
            let mut visited: HashMap<Pos, Pos> = HashMap::new();

            let mut current = HashSet::new();
            current.insert(start);

            loop {
                let mut new_current = HashSet::new();

                for curr_pos in current {
                    let curr_height = topology.at(&curr_pos);

                    for (pos, cell) in topology.neighbours(curr_pos) {
                        if pos != start && !visited.contains_key(&pos) && neighbour_filter(&curr_height, &cell) {
                            visited.insert(pos, curr_pos);
                            new_current.insert(pos);
                        }
                    }
                }

                if let Some(end) = new_current.iter().find(|pos| termination(&topology.at(pos))).cloned() {
                    let mut path = vec![end];

                    let mut current = end;
                    while current != start {
                        current = visited[&current];
                        path.push(current)
                    }

                    path.reverse();
                    return Ok(path);
                }

                if new_current.is_empty() {
                    break;
                }

                current = new_current;
            }

            Err(Error::NoPathFound)
        }
    }
}

fn run_challenge1(content: &str) -> Result<Vec<Pos>, Error> {
    let topology = Topology::parse(content)?;
    walk(
        topology,
        Cell::is_start,
        |curr, neighbour| neighbour.height() <= curr.height() + 1,
        Cell::is_end,
    )
}

fn run_challenge2(content: &str) -> Result<Vec<Pos>, Error> {
    let topology = Topology::parse(content)?;
    walk(
        topology,
        Cell::is_end,
        |curr, neighbour| curr.height() == 0 || neighbour.height() >= curr.height() - 1,
        |c| c.height() == Cell::MIN_HEIGHT,
    )
}

#[derive(Debug, Error)]
enum Error {
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    Nom(#[from] nom::error::Error<String>),
    #[error("Empty input")]
    EmptyInput,
    #[error("All lines should be of the same size")]
    InvalidLineSize,
    #[error("No start found")]
    NoStartFound,
    #[error("No path found")]
    NoPathFound,
}


#[cfg(test)]
mod tests {
    use crate::day12::*;

    #[test]
    fn challenge1_example() -> Result<(), Error> {
        let result = run_challenge1(include_str!("data/day12_example.txt"))?;
        assert_eq!(result.len() - 1, 31);
        Ok(())
    }

    #[test]
    fn challenge1() -> Result<(), Error> {
        let result = run_challenge1(include_str!("data/day12_challenge.txt"))?;
        dbg!(result.len() - 1);
        assert_eq!(result.len() - 1, 352);
        Ok(())
    }

    #[test]
    fn challenge2_example() -> Result<(), Error> {
        let result = run_challenge2(include_str!("data/day12_example.txt"))?;
        assert_eq!(result.len() - 1, 29);
        Ok(())
    }

    #[test]
    fn challenge2() -> Result<(), Error> {
        let result = run_challenge2(include_str!("data/day12_challenge.txt"))?;
        assert_eq!(result.len() - 1, 352);
        Ok(())
    }
}