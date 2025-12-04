#![feature(iter_map_windows)]
#![feature(int_roundings)]

use std::error::Error;
use std::io::prelude::*;

use nom::{
    branch::alt,
    character::complete::{char, i32, newline},
    combinator::map,
    multi::separated_list1,
    sequence::preceded,
    IResult,
};

fn parse_code(input: &str) -> IResult<&str, Vec<i32>> {
    separated_list1(
        newline,
        alt((
            preceded(char('R'), i32),
            map(preceded(char('L'), i32), |s| -s),
        )),
    )(input)
}

fn main() -> Result<(), Box<dyn Error>> {
    let stdin = std::io::stdin();
    let mut stdin = stdin.lock();
    let mut input = String::new();

    stdin.read_to_string(&mut input)?;
    let (_, rots) = parse_code(&input).map_err(|e| format!("Invalid input: {e}"))?;

    let initial: i32 = 50;
    let wrap: i32 = 100;
    let answer = rots
        .iter()
        .scan(initial, |s, r| {
            *s = *s + r;
            Some(*s)
        })
        .filter(|r| r % wrap == 0)
        .count();

    println!("The number of zeros is: {:?}", answer);

    let answer: i32 = rots
        .iter()
        .scan(initial, |s, r| {
            *s = *s + r;
            Some(*s)
        })
        .map_windows(|[a, b]| (a.div_floor(wrap) - b.div_floor(wrap)).abs())
        .sum();

    println!("The number of zero crossings is: {:?}", answer);

    return Ok(());
}
