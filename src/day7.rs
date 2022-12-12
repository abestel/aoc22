use camino::Utf8PathBuf;
use nom::{
    Finish,
    IResult,
    branch::alt,
    character::complete,
    bytes::complete::{tag, take_while1},
    combinator::{all_consuming, map},
    sequence::{preceded, separated_pair},
};
use std::{
    cell::RefCell,
    collections::HashMap,
    fmt,
    fs::File,
    io::{BufRead, BufReader},
    iter,
    path::Path,
    rc::Rc,
};
use thiserror::Error;

#[derive(Debug)]
struct List;

fn parse_list(i: &str) -> IResult<&str, List> {
    map(tag("ls"), |_| List)(i)
}

#[derive(Debug)]
struct ChangeDirectory(Utf8PathBuf);

fn parse_path(i: &str) -> IResult<&str, Utf8PathBuf> {
    map(
        take_while1(|c: char| c.is_alphabetic() || c == '.' || c == '/'),
        Utf8PathBuf::from,
    )(i)
}

fn parse_change_directory(i: &str) -> IResult<&str, ChangeDirectory> {
    map(preceded(tag("cd "), parse_path), ChangeDirectory)(i)
}

#[derive(Debug)]
enum Command {
    List(List),
    ChangeDirectory(ChangeDirectory),
}

fn parse_command(i: &str) -> IResult<&str, Command> {
    let (i, _) = tag("$ ")(i)?;
    alt(
        (
            map(parse_list, Command::List),
            map(parse_change_directory, Command::ChangeDirectory)
        )
    )(i)
}

#[derive(Debug)]
enum Entry {
    Dir(Utf8PathBuf),
    File(u64, Utf8PathBuf),
}

fn parse_entry(i: &str) -> IResult<&str, Entry> {
    let parse_file = map(
        separated_pair(complete::u64, tag(" "), parse_path),
        |(size, path)| Entry::File(size, path),
    );

    let parse_dir = map(
        preceded(tag("dir "), parse_path),
        Entry::Dir,
    );

    alt((parse_file, parse_dir))(i)
}

#[derive(Debug)]
enum Line {
    Command(Command),
    Entry(Entry),
}

fn parse_line(i: &str) -> IResult<&str, Line> {
    alt(
        (
            map(parse_entry, Line::Entry),
            map(parse_command, Line::Command),
        )
    )(i)
}

#[derive(Debug, Error)]
enum Error {
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error("Impossible to parse '{0}' because '{1:?}'")]
    Nom(String, nom::error::ErrorKind),
    #[error("No directory found")]
    NoDirectoryFound,
}


struct Node {
    parent: Option<NodeHandle>,
    name: Utf8PathBuf,
    size: u64,
    children: HashMap<Utf8PathBuf, NodeHandle>,
}

impl Node {
    fn new_dir(name: Utf8PathBuf, parent: Option<NodeHandle>) -> Node {
        Node {
            parent,
            name,
            size: 0_u64,
            children: HashMap::new(),
        }
    }
    fn new_file(name: Utf8PathBuf, size: u64, parent: Option<NodeHandle>) -> Node {
        Node {
            parent,
            name,
            size,
            children: HashMap::new(),
        }
    }

    fn is_dir(&self) -> bool {
        self.size == 0
    }

    fn total_size(&self) -> u64 {
        self.size + self.children
            .values()
            .map(|child| child.borrow().total_size())
            .sum::<u64>()
    }
}

type NodeHandle = Rc<RefCell<Node>>;

fn all_dirs(node: NodeHandle) -> Box<dyn Iterator<Item=NodeHandle>> {
    #[allow(clippy::needless_collect)]
        let children = node.borrow().children.values().cloned().collect::<Vec<_>>();

    Box::new(
        iter::once(node).chain(
            children
                .into_iter()
                .filter_map(|c| {
                    if c.borrow().is_dir() {
                        Some(all_dirs(c))
                    } else {
                        None
                    }
                })
                .flatten()
        )
    )
}

struct PrettyNode<'a>(&'a NodeHandle);

impl<'a> fmt::Debug for PrettyNode<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let this = self.0.borrow();
        if this.size == 0 {
            writeln!(f, "{} (dir)", this.name)?;
        } else {
            writeln!(f, "{} (file, size={})", this.name, this.size)?;
        }

        for child in this.children.values() {
            // not very efficient at all, but shrug
            for (index, line) in format!("{:?}", PrettyNode(child)).lines().enumerate() {
                if index == 0 {
                    writeln!(f, "{line}")?;
                } else {
                    writeln!(f, "  {line}")?;
                }
            }
        }
        Ok(())
    }
}


fn read_input<P>(path: P) -> Result<NodeHandle, Error>
    where P: AsRef<Path> {
    let file = File::open(path)?;

    let root = Rc::new(RefCell::new(Node::new_dir("/".parse().unwrap(), None)));
    let mut node = root.clone();

    for line in BufReader::new(file).lines() {
        let line = line?;
        let (_, line) = all_consuming(parse_line)(&line)
            .finish()
            .map_err(|e| Error::Nom(line.clone(), e.code))?;

        println!("{:?}", line);

        match line {
            Line::Command(command) =>
                match command {
                    Command::List(_) => (),
                    Command::ChangeDirectory(ChangeDirectory(name)) => {
                        match name.as_str() {
                            "/" => node = root.clone(),
                            ".." => node = node.clone().borrow().parent.clone().unwrap_or_else(|| root.clone()),
                            _ => node = node.clone().borrow_mut().children
                                .entry(name.clone())
                                .or_insert_with(||
                                    Rc::new(
                                        RefCell::new(
                                            Node::new_dir(name.clone(), Some(node.clone()))
                                        )
                                    )
                                ).clone()
                        };
                    }
                },
            Line::Entry(entry) =>
                match entry {
                    Entry::Dir(name) => {
                        node.borrow_mut().children
                            .entry(name.clone())
                            .or_insert_with(||
                                Rc::new(
                                    RefCell::new(
                                        Node::new_dir(name.clone(), Some(node.clone()))
                                    )
                                )
                            );
                    }
                    Entry::File(size, name) => {
                        node.borrow_mut().children
                            .entry(name.clone())
                            .or_insert_with(||
                                Rc::new(
                                    RefCell::new(
                                        Node::new_file(name.clone(), size, Some(node.clone()))
                                    )
                                )
                            );
                    }
                }
        }
    }

    println!("{:#?}", PrettyNode(&root));

    Ok(root)
}

fn run_challenge1<P>(path: P) -> Result<u64, Error>
    where P: AsRef<Path> {
    let nodes = read_input(path)?;

    let sum = all_dirs(nodes)
        .map(|d| d.borrow().total_size())
        .filter(|&s| s <= 100_000)
        .sum::<u64>();

    Ok(sum)
}

fn run_challenge2<P>(path: P) -> Result<u64, Error>
    where P: AsRef<Path> {
    let root = read_input(path)?;

    let total_space = 70000000_u64;
    let used_space = root.borrow().total_size();
    let free_space = total_space - used_space;
    let needed_free_space = 30000000_u64;
    let minimum_space_to_free = needed_free_space - free_space;

    let removed_dir_size = all_dirs(root)
        .map(|d| d.borrow().total_size())
        .filter(|&s| s >= minimum_space_to_free)
        .min();

    removed_dir_size.ok_or(Error::NoDirectoryFound)
}

#[cfg(test)]
mod tests {
    use crate::day7::*;

    #[test]
    fn challenge1_example() -> Result<(), Error> {
        let sum = run_challenge1("resources/day7_example.txt")?;
        assert_eq!(sum, 95437);
        Ok(())
    }

    #[test]
    fn challenge1() -> Result<(), Error> {
        let sum = run_challenge1("resources/day7_challenge.txt")?;
        dbg!(sum);
        Ok(())
    }

    #[test]
    fn challenge2_example() -> Result<(), Error> {
        let size = run_challenge2("resources/day7_example.txt")?;
        assert_eq!(size, 24933642);
        Ok(())
    }

    #[test]
    fn challenge2() -> Result<(), Error> {
        let sum = run_challenge2("resources/day7_challenge.txt")?;
        dbg!(sum);
        Ok(())
    }
}