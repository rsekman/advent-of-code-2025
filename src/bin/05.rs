use std::error::Error;
use std::io::prelude::*;

use std::cmp::max;

use nom::{
    character::complete::{char, newline, u64},
    multi::{many1, separated_list1},
    sequence::separated_pair,
    IResult, Parser,
};

type IngredientRange = (u64, u64);

fn id_range(input: &str) -> IResult<&str, IngredientRange> {
    separated_pair(u64, char('-'), u64).parse(input)
}

fn ingredients(input: &str) -> IResult<&str, Vec<u64>> {
    separated_list1(newline, u64).parse(input)
}

fn parse_input(input: &str) -> IResult<&str, (Vec<IngredientRange>, Vec<u64>)> {
    separated_pair(
        separated_list1(newline, id_range),
        many1(newline),
        ingredients,
    )
    .parse(input)
}

fn n_fresh(ranges: &Vec<IngredientRange>, ingredients: &Vec<u64>) -> usize {
    ingredients
        .into_iter()
        .filter(|&i| ranges.iter().any(|&(lo, hi)| lo <= *i && *i <= hi))
        .count()
}

fn overlap((_, a_hi): IngredientRange, (b_lo, _): IngredientRange) -> bool {
    b_lo <= a_hi
}

fn merge_ranges(ranges: &Vec<IngredientRange>) -> Vec<IngredientRange> {
    let mut out: Vec<IngredientRange> = vec![];

    let mut sorted_ranges = ranges.clone();
    sorted_ranges.sort_by_key(|&(lo, _)| lo);

    for r in sorted_ranges.into_iter() {
        match out.last_mut() {
            None => out.push(r),
            Some(p) => {
                if overlap(*p, r) {
                    *p = (p.0, max(r.1, p.1));
                } else {
                    out.push(r)
                }
            }
        }
    }
    out
}

fn main() -> Result<(), Box<dyn Error>> {
    let stdin = std::io::stdin();
    let mut stdin = stdin.lock();
    let mut input = String::new();

    stdin.read_to_string(&mut input)?;
    let (_, (ranges, ingredients)) =
        parse_input(&input).map_err(|e| format!("Invalid input: {e}"))?;

    let answer = n_fresh(&ranges, &ingredients);

    println!("The number of fresh ingredients is {answer}.");

    let merged = merge_ranges(&ranges);

    let total_fresh: u64 = merged.into_iter().map(|(lo, hi)| hi - lo + 1).sum();
    println!("The total number of ingredients considered fresh is {total_fresh}.");

    return Ok(());
}
