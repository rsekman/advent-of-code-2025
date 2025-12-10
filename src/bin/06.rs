use std::error::Error;
use std::io::prelude::*;

use itertools::Itertools;
use nom::{
    branch::alt,
    character::complete::{char, multispace1, newline, space0, space1, u64},
    combinator::value,
    multi::separated_list1,
    sequence::{delimited, separated_pair},
    IResult, Parser,
};

#[derive(Debug, Clone, Copy)]
enum Operation {
    Add,
    Multiply,
}

fn parse_operator(input: &str) -> IResult<&str, Operation> {
    alt((
        value(Operation::Add, char('+')),
        value(Operation::Multiply, char('*')),
    ))
    .parse(input)
}

fn parse_operands(input: &str) -> IResult<&str, Vec<u64>> {
    delimited(space0, separated_list1(space1, u64), space0).parse(input)
}

fn parse_input(input: &str) -> IResult<&str, (Vec<Vec<u64>>, Vec<Operation>)> {
    separated_pair(
        separated_list1(newline, parse_operands),
        newline,
        delimited(space0, separated_list1(space1, parse_operator), space0),
    )
    .parse(input)
}

fn parse_transposed(input: &str) -> IResult<&str, Vec<(Vec<u64>, Operation)>> {
    separated_list1(
        multispace1,
        separated_pair(
            separated_list1(newline, delimited(space0, u64, space0)),
            space0,
            parse_operator,
        ),
    )
    .parse(input)
}

fn main() -> Result<(), Box<dyn Error>> {
    let stdin = std::io::stdin();
    let mut stdin = stdin.lock();
    let mut input = String::new();

    stdin.read_to_string(&mut input)?;
    let (_, (operands, operators)) =
        parse_input(&input).map_err(|e| format!("Invalid input: {e}"))?;

    let mut columns: Vec<_> = operands.into_iter().map(Vec::into_iter).collect();

    let answer: u64 = operators
        .into_iter()
        .map(|o| -> u64 {
            let values = columns.iter_mut().filter_map(Iterator::next);
            match o {
                Operation::Add => values.sum(),
                Operation::Multiply => values.product(),
            }
        })
        .sum();

    println!("The answer is {answer}");

    // Transpose columns and lines
    let mut transposed = String::new();
    let mut lines: Vec<_> = input.lines().map(|l| l.chars()).collect();
    loop {
        let mut x = lines
            .iter_mut()
            .map(|l| l.next_back().map(|c| transposed.push(c)));
        if x.contains(&None) {
            break;
        } else {
            transposed.push('\n');
        }
    }
    let (_, parsed) = parse_transposed(&transposed).map_err(|e| format!("Invalid input: {e}"))?;

    let answer: u64 = parsed
        .into_iter()
        .map(|(vs, op)| -> u64 {
            match op {
                Operation::Add => vs.iter().sum(),
                Operation::Multiply => vs.iter().product(),
            }
        })
        .sum();

    println!("The answer (transposed) is {answer}");

    return Ok(());
}
