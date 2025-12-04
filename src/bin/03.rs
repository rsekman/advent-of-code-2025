#![feature(iter_advance_by)]

use std::error::Error;
use std::io::prelude::*;

use itertools::Itertools;

use nom::{
    character::complete::{anychar, newline, u32},
    combinator::{map_parser, recognize},
    multi::{many1, separated_list1},
    IResult,
};

fn parse_batteries(input: &str) -> IResult<&str, Vec<Vec<u32>>> {
    separated_list1(newline, many1(map_parser(recognize(anychar), u32)))(input)
}

fn max_joltage(bats: &Vec<u32>, n: u32) -> Option<u64> {
    let mut joltage: u64 = 0;
    let mut pos = 0;
    for d in (0..n).rev() {
        let digit = *(bats.iter().dropping(pos).dropping_back(d as usize).max()?);
        pos += bats.iter().dropping(pos).position(|&x| x == digit)? + 1;
        joltage += 10_u64.pow(d) * (digit as u64);
    }
    Some(joltage)
}

fn main() -> Result<(), Box<dyn Error>> {
    let stdin = std::io::stdin();
    let mut stdin = stdin.lock();
    let mut input = String::new();

    stdin.read_to_string(&mut input)?;
    let (_, bats) = parse_batteries(&input).map_err(|e| format!("Invalid input: {e}"))?;

    let max_joltages: Vec<_> = bats.iter().filter_map(|v| max_joltage(v, 2)).collect();
    let total_joltage: u64 = max_joltages.iter().sum();

    println!("The maximum total joltage (2 batteries) is {total_joltage}");

    let max_joltages: Vec<_> = bats.iter().filter_map(|v| max_joltage(v, 12)).collect();
    let total_joltage: u64 = max_joltages.iter().sum();

    println!("The maximum total joltage (12 batteries) is {total_joltage}");

    return Ok(());
}
