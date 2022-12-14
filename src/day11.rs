use nom::{
    Finish,
    IResult,
    branch::alt,
    bytes::complete::tag,
    character::complete,
    combinator::{all_consuming, map, value},
    multi::separated_list1,
    sequence::{preceded, tuple},
};
use thiserror::Error;

#[derive(Clone, Copy, Debug)]
enum Var {
    Old,
    Num(u64),
}

impl Var {
    fn parse(i: &str) -> IResult<&str, Self> {
        let old_parser = value(Var::Old, tag("old"));
        let num_parser = map(complete::u64, Var::Num);

        alt((old_parser, num_parser))(i)
    }

    fn apply(self, old: u64) -> u64 {
        match self {
            Var::Old => old,
            Var::Num(num) => num
        }
    }
}

#[derive(Clone, Copy, Debug)]
enum Operator {
    Add,
    Mul,
}

impl Operator {
    fn parse(i: &str) -> IResult<&str, Self> {
        alt((
            value(Operator::Add, complete::char('+')),
            value(Operator::Mul, complete::char('*')),
        ))(i)
    }
}

#[derive(Clone, Copy, Debug)]
enum Operation {
    Add(Var, Var),
    Mul(Var, Var),
}

impl Operation {
    fn parse(i: &str) -> IResult<&str, Self> {
        preceded(
            tag("new ="),
            map(
                tuple((
                    complete::space0,
                    Var::parse,
                    complete::space0,
                    Operator::parse,
                    complete::space0,
                    Var::parse,
                )), |(_, left, _, operator, _, right)| match operator {
                    Operator::Add => Operation::Add(left, right),
                    Operator::Mul => Operation::Mul(left, right),
                },
            ),
        )(i)
    }

    fn apply(self, old: u64) -> u64 {
        match self {
            Operation::Add(left, right) => left.apply(old) + right.apply(old),
            Operation::Mul(left, right) => left.apply(old) * right.apply(old),
        }
    }
}

#[derive(Clone, Copy, Debug)]
struct Test {
    divisible_by: u64,
    if_true_send_to: usize,
    if_false_send_to: usize,
}

impl Test {
    fn parse(i: &str) -> IResult<&str, Self> {
        map(
            tuple((
                tag("divisible by"),
                complete::space1,
                complete::u64,
                complete::line_ending,
                complete::space1,
                tag("If true: throw to monkey"),
                complete::space1,
                complete::u32,
                complete::line_ending,
                complete::space1,
                tag("If false: throw to monkey"),
                complete::space1,
                complete::u32,
            )),
            |(_, _, divisible_by, _, _, _, _, if_true_send_to, _, _, _, _, if_false_send_to)| Test {
                divisible_by,
                if_true_send_to: if_true_send_to as usize,
                if_false_send_to: if_false_send_to as usize,
            },
        )(i)
    }
}

#[derive(Debug, Clone)]
struct Monkey {
    inspected: u64,
    index: u32,
    items: Vec<u64>,
    operation: Operation,
    test: Test,
}

impl Monkey {
    fn parse(i: &str) -> IResult<&str, Self> {
        let monkey_index_parser = map(
            tuple((
                tag("Monkey"),
                complete::space1,
                complete::u32,
                complete::char(':'),
            )),
            |(_, _, index, _)| index,
        );

        let starting_items_parser =
            separated_list1(
                tuple((complete::char(','), complete::space0)),
                complete::u64,
            );

        map(
            tuple((
                monkey_index_parser,
                complete::line_ending,
                complete::space1,
                tag("Starting items:"),
                complete::space1,
                starting_items_parser,
                complete::line_ending,
                complete::space1,
                tag("Operation:"),
                complete::space1,
                Operation::parse,
                complete::line_ending,
                complete::space1,
                tag("Test:"),
                complete::space1,
                Test::parse,
                complete::line_ending,
            )),
            |(index, _, _, _, _, items, _, _, _, _, operation, _, _, _, _, test, _)| Monkey { inspected: 0, index, items, operation, test },
        )(i)
    }
}

fn read_input(content: &str) -> Result<Vec<Monkey>, Error> {
    let (_, mut monkeys) = all_consuming(separated_list1(complete::line_ending, Monkey::parse))(content)
        .map_err(|e| e.to_owned())
        .finish()?;

    monkeys.sort_by_key(|x| x.index);

    Ok(monkeys)
}

fn run_loop(iterations: usize, worry_level_divider: u64, mut monkeys: Vec<Monkey>) -> Vec<Monkey> {
    let divisor_product = monkeys.iter().map(|m| m.test.divisible_by).product::<u64>();

    for _ in 0..iterations {
        for m in 0..monkeys.len() {
            let Monkey { operation, test, items, .. } = monkeys[m].clone();

            monkeys[m].inspected += monkeys[m].items.len() as u64;
            monkeys[m].items.clear();

            for mut item in items.iter().cloned() {
                item %= divisor_product;
                item = operation.apply(item);
                item /= worry_level_divider;

                if item % test.divisible_by == 0 {
                    monkeys[test.if_true_send_to as usize].items.push(item);
                } else {
                    monkeys[test.if_false_send_to as usize].items.push(item);
                }
            }
        }
    }

    println!("{:?}", monkeys);

    monkeys
}

fn run_challenge1(content: &str) -> Result<u64, Error> {
    let monkeys = read_input(content)?;
    let monkeys = run_loop(20, 3, monkeys);

    let mut inspected = monkeys.iter().map(|m| m.inspected).collect::<Vec<_>>();
    inspected.sort();

    Ok(inspected.iter().rev().take(2).product())
}

fn run_challenge2(content: &str) -> Result<u64, Error> {
    let monkeys = read_input(content)?;
    let monkeys = run_loop(10_000, 1, monkeys);

    let mut inspected = monkeys.iter().map(|m| m.inspected).collect::<Vec<_>>();
    inspected.sort();

    Ok(inspected.iter().rev().take(2).product())
}

#[derive(Debug, Error)]
enum Error {
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    Nom(#[from] nom::error::Error<String>),
}


#[cfg(test)]
mod tests {
    use crate::day11::*;

    #[test]
    fn challenge1_example() -> Result<(), Error> {
        let result = run_challenge1(include_str!("data/day11_example.txt"))?;
        assert_eq!(result, 10605);
        Ok(())
    }

    #[test]
    fn challenge1() -> Result<(), Error> {
        let result = run_challenge1(include_str!("data/day11_challenge.txt"))?;
        dbg!(result);
        Ok(())
    }

    #[test]
    fn challenge2_example() -> Result<(), Error> {
        let result = run_challenge2(include_str!("data/day11_example.txt"))?;
        assert_eq!(result, 2713310158);
        Ok(())
    }

    #[test]
    fn challenge2() -> Result<(), Error> {
        let result = run_challenge2(include_str!("data/day11_challenge.txt"))?;
        println!("{}", result);
        Ok(())
    }
}