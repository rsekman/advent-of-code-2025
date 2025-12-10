#![feature(string_into_chars)]
#![feature(cmp_minmax)]

use std::cmp::{max, min, minmax, Reverse};
use std::collections::BTreeMap;
use std::error::Error;
use std::io::prelude::*;

use itertools::{Itertools, MinMaxResult};

use rayon::prelude::*;

use nom::{
    character::complete::{char, newline, u64},
    multi::separated_list1,
    sequence::separated_pair,
    IResult, Parser,
};

type Coordinates = (u64, u64);

fn parse_input(input: &str) -> IResult<&str, Vec<Coordinates>> {
    separated_list1(newline, separated_pair(u64, char(','), u64)).parse(input)
}

fn intersects(bottom: u64, top: u64, y: u64) -> bool {
    2 * bottom <= 2 * y + 1 && 2 * y + 1 <= 2 * top
}

fn area((x1, y1): Coordinates, (x2, y2): Coordinates) -> u64 {
    (u64::abs_diff(x1, x2) + 1) * (u64::abs_diff(y1, y2) + 1)
}

fn all_red_or_green(
    a: Coordinates,
    b: Coordinates,
    intervals: &BTreeMap<u64, Vec<(u64, u64)>>,
) -> bool {
    let [top, bottom] = minmax(a.1, b.1);
    let [left, right] = minmax(a.0, b.0);
    (top..bottom).all(|y| {
        intervals
            .get(&y)
            .and_then(|v| v.iter().find(|(a, b)| *a <= left && right <= *b))
            .is_some()
    })
}

fn main() -> Result<(), Box<dyn Error>> {
    let stdin = std::io::stdin();
    let mut stdin = stdin.lock();
    let mut input = String::new();

    stdin.read_to_string(&mut input)?;
    let (_, reds) = parse_input(&input).map_err(|e| format!("Invalid input: {e}"))?;

    let mut squares: Vec<(_, _)> = reds.iter().tuple_combinations().collect();
    squares.sort_by_key(|&(&a, &b)| Reverse(area(a, b)));
    let answer = area(*squares[0].0, *squares[0].1);

    println!("Maximum area: {answer}",);

    let mut verticals = reds
        .iter()
        .cloned()
        .circular_tuple_windows()
        .filter(|((x1, _), (x2, _))| x1 == x2)
        .map(|((x, y1), (_, y2))| (x, min(y1, y2), max(y1, y2)))
        .collect_vec();
    verticals.sort_by_key(|(x, _, _)| *x);

    let (top, bottom) = match reds.iter().minmax_by_key(|(_, y)| *y) {
        MinMaxResult::MinMax((_, y1), (_, y2)) => (*y1, *y2),
        MinMaxResult::OneElement((y, _)) => (*y, *y),
        MinMaxResult::NoElements => unreachable!("No reds"),
    };

    let intervals: BTreeMap<u64, Vec<(u64, u64)>> = (top..=bottom)
        .map(|y| {
            (
                y,
                verticals
                    .iter()
                    .filter(|(_, bottom, top)| intersects(*bottom, *top, y))
                    .map(|(x, _, _)| *x)
                    .tuple_windows::<(_, _)>()
                    .step_by(2)
                    .collect_vec(),
            )
        })
        .collect();

    let n = squares.len();
    let (considered, answer) = squares
        .into_par_iter()
        .enumerate()
        .find_first(|&(_, (&a, &b))| all_red_or_green(a, b, &intervals))
        .map(|(n, (&a, &b))| (n, area(a, b)))
        .ok_or("No reds?")?;

    println!("Maximum all red/green area: {answer} (considered {considered}/{n} squares)",);

    return Ok(());
}
