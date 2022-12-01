use std::{
    fs::File,
    io::{BufReader, BufRead},
    path::Path,
};

fn read_input<P>(path: P) -> Result<Vec<Vec<u32>>, anyhow::Error>
    where P: AsRef<Path> {
    let file = File::open(path)?;
    let lines = BufReader::new(file).lines();

    let mut elves: Vec<Vec<u32>> = Vec::new();
    let mut current: Vec<u32> = Vec::new();

    for line in lines {
        let line = line?;
        if line.is_empty() {
            elves.push(current);
            current = Vec::new();
        } else {
            let weight: u32 = line.parse()?;
            current.push(weight);
        }
    }

    if !current.is_empty() {
        elves.push(current);
    }

    Ok(elves)
}

fn compute_calories(elves: Vec<Vec<u32>>) -> Vec<u32> {
    elves
        .into_iter()
        .map(|elf| elf.into_iter().sum::<u32>())
        .collect()
}

fn max_calories(elves_calories: Vec<u32>) -> Option<u32> {
    elves_calories.into_iter().max()
}

fn run_challenge1<P>(path: P) -> Result<u32, anyhow::Error>
    where P: AsRef<Path> {
    let elves = read_input(path)?;
    let elves = compute_calories(elves);
    Ok(max_calories(elves).unwrap_or_default())
}

fn run_challenge2<P>(path: P) -> Result<u32, anyhow::Error>
    where P: AsRef<Path> {
    let elves = read_input(path)?;
    let mut elves = compute_calories(elves);
    elves.sort();

    Ok(elves.iter().rev().take(3).sum())
}

#[cfg(test)]
mod tests {
    use crate::day1::*;

    #[test]
    fn challenge1_example() -> Result<(), anyhow::Error> {
        let max = run_challenge1("resources/day1_example.txt")?;
        assert_eq!(max, 24000);
        Ok(())
    }

    #[test]
    fn challenge1() -> Result<(), anyhow::Error> {
        let max = run_challenge1("resources/day1_challenge.txt")?;
        println!("{}", max);
        Ok(())
    }

    #[test]
    fn challenge2_example() -> Result<(), anyhow::Error> {
        let top3 = run_challenge2("resources/day1_example.txt")?;
        assert_eq!(top3, 45000);
        Ok(())
    }

    #[test]
    fn challenge2() -> Result<(), anyhow::Error> {
        let top3 = run_challenge2("resources/day1_challenge.txt")?;
        println!("{}", top3);
        Ok(())
    }
}