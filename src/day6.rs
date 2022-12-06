use std::{
    collections::HashSet,
    fs::File,
    io::{BufReader, BufRead, Lines},
    path::Path,
};
use thiserror::Error;

fn find_packet_distinct_chars(s: &str, packet_size: usize) -> Result<usize, Error> {
    s.as_bytes()
        .windows(packet_size)
        .enumerate()
        .find(|(_, chars)| {
            let mut set: HashSet<u8> = HashSet::new();
            for c in chars.iter() {
                if !set.insert(*c) {
                    break;
                }
            }

            set.len() == chars.len()
        })
        .map(|(index, chars)| index + chars.len())
        .ok_or_else(|| Error::NoPacketStart(s.to_string()))
}

fn find_packet_start(s: &str) -> Result<usize, Error> {
    find_packet_distinct_chars(s, 4)
}

fn find_message_start(s: &str) -> Result<usize, Error> {
    find_packet_distinct_chars(s, 14)
}

#[derive(Debug, Error)]
enum Error {
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error("No packet start found in '{0}'")]
    NoPacketStart(String),
}

fn read_input<P>(path: P) -> Result<Lines<BufReader<File>>, Error>
    where P: AsRef<Path> {
    let file = File::open(path)?;
    Ok(BufReader::new(file).lines())
}

fn run_challenge1<P>(path: P) -> Result<Vec<usize>, Error>
    where P: AsRef<Path> {
    let lines = read_input(path)?;

    let mut indexes: Vec<usize> = Vec::new();
    for line in lines {
        indexes.push(find_packet_start(&line?)?);
    }

    Ok(indexes)
}

fn run_challenge2<P>(path: P) -> Result<Vec<usize>, Error>
    where P: AsRef<Path> {
    let lines = read_input(path)?;

    let mut indexes: Vec<usize> = Vec::new();
    for line in lines {
        indexes.push(find_message_start(&line?)?);
    }

    Ok(indexes)
}

#[cfg(test)]
mod tests {
    use crate::day6::*;

    #[test]
    fn challenge1_example() -> Result<(), Error> {
        let result = run_challenge1("resources/day6_example.txt")?;
        assert_eq!(result, vec![7, 5, 6, 10, 11]);
        Ok(())
    }

    #[test]
    fn challenge1() -> Result<(), Error> {
        let result = run_challenge1("resources/day6_challenge.txt")?;
        println!("{:?}", result);
        Ok(())
    }

    #[test]
    fn challenge2_example() -> Result<(), Error> {
        let result = run_challenge2("resources/day6_example.txt")?;
        assert_eq!(result, vec![19, 23, 23, 29, 26]);
        Ok(())
    }

    #[test]
    fn challenge2() -> Result<(), Error> {
        let result = run_challenge2("resources/day6_challenge.txt")?;
        println!("{:?}", result);
        Ok(())
    }
}