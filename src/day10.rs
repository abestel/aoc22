use nom::{
    Finish,
    IResult,
    branch::alt,
    bytes::complete::tag,
    character::complete,
    combinator::{all_consuming, map, value},
    sequence::preceded,
};
use std::{
    collections::VecDeque,
    fmt::{self, Formatter},
    fs::File,
    io::{BufReader, BufRead},
    path::Path
};
use thiserror::Error;

#[derive(Clone, Debug)]
enum Command {
    NoOp,
    Addx(i64),
}

impl Command {
    fn parse(i: &str) -> IResult<&str, Command> {
        let parse_noop = value(Command::NoOp, tag("noop"));
        let parse_addx = map(preceded(tag("addx "), complete::i64), Command::Addx);

        alt((
            parse_noop,
            parse_addx
        ))(i)
    }

    fn cycles(&self) -> usize {
        match self {
            Command::NoOp => 1,
            Command::Addx(_) => 2,
        }
    }
}

fn read_input<P>(path: P) -> Result<VecDeque<Command>, Error>
    where P: AsRef<Path> {
    let file = File::open(path)?;

    let mut commands = VecDeque::new();
    for line in BufReader::new(file).lines() {
        let line = line?;
        let (_, command) = all_consuming(Command::parse)(&line)
            .finish()
            .map_err(|e| Error::Nom(line.clone(), e.code))?;

        commands.push_back(command);
    }

    Ok(commands)
}

#[derive(Debug)]
struct Machine {
    register: i64,
    crt: [[bool; 40]; 6],
}

impl Machine {
    fn new() -> Self {
        Self { register: 1, crt: [[false; 40]; 6] }
    }

    fn is_lighten_pixel(&self, x: i64) -> bool {
        self.register - 1 <= x && x <= self.register + 1
    }
}

impl fmt::Display for Machine {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        for line in self.crt {
            for pixel in line {
                f.write_str(if pixel { "#" } else { "." })?;
            }
            f.write_str("\n")?;
        }

        Ok(())
    }
}

enum Action {
    AwaitCommand,
    Defer(Command, usize),
}

fn run_loop(mut commands: VecDeque<Command>) -> Result<(i64, Machine), Error> {
    let mut machine = Machine::new();

    let mut current_action = Action::AwaitCommand;
    let mut cycle = 1_usize;

    let mut strength = 0_i64;

    loop {
        let x = (cycle - 1) % 40;
        if machine.is_lighten_pixel(x as i64) {
            machine.crt[(cycle - 1) / 40][x] = true;
        }

        if cycle == 20 || cycle > 20 && (cycle - 20) % 40 == 0 {
            let cycle_strength = cycle as i64 * machine.register;
            strength += cycle_strength;
            println!("Cycle {} | X={} | Cycle Strength={} | Total Strength={}", cycle, machine.register, cycle_strength, strength);
        }

        match current_action {
            Action::AwaitCommand => match commands.pop_front() {
                None => break,
                Some(command) => {
                    let cycles = command.cycles();
                    if cycles > 1 {
                        current_action = Action::Defer(command, cycles - 1);
                    } else {
                        current_action = Action::AwaitCommand;
                    }
                }
            }
            Action::Defer(command, cycles) =>
                if cycles == 1 {
                    match command {
                        Command::NoOp => (),
                        Command::Addx(delta) => machine.register += delta,
                    }
                    current_action = Action::AwaitCommand;
                } else {
                    current_action = Action::Defer(command, cycles - 1);
                }
        }


        cycle += 1;
    }

    Ok((strength, machine))
}

fn run_challenge1<P>(path: P) -> Result<i64, Error>
    where P: AsRef<Path> {
    let commands = read_input(path)?;
    Ok(run_loop(commands)?.0)
}

fn run_challenge2<P>(path: P) -> Result<Machine, Error>
    where P: AsRef<Path> {
    let commands = read_input(path)?;
    Ok(run_loop(commands)?.1)
}

#[derive(Debug, Error)]
enum Error {
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error("Impossible to parse '{0}' because '{1:?}'")]
    Nom(String, nom::error::ErrorKind),
}


#[cfg(test)]
mod tests {
    use crate::day10::*;

    #[test]
    fn challenge1_example() -> Result<(), Error> {
        let result = run_challenge1("resources/day10_example.txt")?;
        assert_eq!(result, 13140);
        Ok(())
    }

    #[test]
    fn challenge1() -> Result<(), Error> {
        let result = run_challenge1("resources/day10_challenge.txt")?;
        dbg!(result);
        Ok(())
    }

    #[test]
    fn challenge2_example() -> Result<(), Error> {
        let result = run_challenge2("resources/day10_example.txt")?;
        println!("{}", result);
        Ok(())
    }

    #[test]
    fn challenge2() -> Result<(), Error> {
        let result = run_challenge2("resources/day10_challenge.txt")?;
        println!("{}", result);
        Ok(())
    }
}