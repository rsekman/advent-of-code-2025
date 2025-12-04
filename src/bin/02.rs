#![feature(iter_map_windows)]
#![feature(int_roundings)]

use std::cmp::{max, min};
use std::error::Error;
use std::io::prelude::*;

use nom::{
    character::complete::{char, u64},
    multi::separated_list1,
    sequence::separated_pair,
    IResult,
};

fn parse_ids(input: &str) -> IResult<&str, Vec<(u64, u64)>> {
    separated_list1(char(','), separated_pair(u64, char('-'), u64))(input)
}

// Sum the invalid IDs between start and end, where an ID is invalid iff it its decimal
// representation is an n-fold repetition
fn sum_invalids(start: u64, end: u64, n: u32) -> u64 {
    // number of digits in start
    let d = start.ilog(10) + 1;
    // Invalid IDs will have p repetitions of n-digit numbers
    // This implicitly assumes that the interval end/start < 10^n
    let p = d.div_ceil(n);

    let terms: Vec<_> = (0..(p * n)).step_by(p as usize).collect();
    let sep = terms.iter().map(|q| 10_u64.pow(*q)).sum();

    // This ensures that if the number of digits in start is not divisble by n, we start at the first
    // number with n * p digits
    // E.g. if n = 2 and start = 109_0286, we start at 1000_1000
    let first = ((p - 1)..(p * n))
        .step_by(p as usize)
        .map(|q| 10_u64.pow(q))
        .sum();
    let start = max(start.div_ceil(sep) * sep, first);

    let end = min(end, 10_u64.pow(n * p) - 1);

    if start > end {
        0
    } else {
        // The invalid IDs form an arithmetic sequence with initial value start, difference sep,
        // and this many elements
        let count = (end + 1 - start).div_ceil(sep);
        // The sum of such a sequence is the number of elements times the average element
        ((start + (start + (count - 1) * sep)) * count) / 2
        // The division by 2 goes last to avoid rounding
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let stdin = std::io::stdin();
    let mut stdin = stdin.lock();
    let mut input = String::new();

    stdin.read_to_string(&mut input)?;
    let (_, spans) = parse_ids(&input).map_err(|e| format!("Invalid input: {e}"))?;

    let answer: u64 = spans
        .iter()
        .map(|(start, end)| sum_invalids(*start, *end, 2))
        .sum();

    println!("The sum of invalid IDs (two-fold repetition only) is {answer}");

    // It is enough to search for p-fold repeat where p is prime, but if q is composite a q-fold
    // repeat will be found by searches for each of its prime factors, so it will be double
    // counted. For our inputs the maximum number of digits is 10, so it is enough to check the
    // single-digit primes, and the only composite number we need to exclude is 6.
    let inclusion: u64 = vec![2, 3, 5, 7]
        .into_iter()
        .map(|n| -> u64 {
            spans
                .iter()
                .map(|(start, end)| sum_invalids(*start, *end, n))
                .sum()
        })
        .sum();
    let exclusion: u64 = spans
        .iter()
        .map(|(start, end)| sum_invalids(*start, *end, 6))
        .sum();

    println!(
        "The sum of invalid IDs (any number of repetitions) is {}",
        inclusion - exclusion
    );

    return Ok(());
}
