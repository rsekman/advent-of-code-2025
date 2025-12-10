use std::error::Error;
use std::io::prelude::*;

use std::collections::BTreeSet;

use nom::{
    branch::alt,
    character::complete::{char, newline, space0, u64, usize},
    combinator::{map, value},
    multi::{many1, separated_list1},
    sequence::delimited,
    IResult, Parser,
};

use itertools::Itertools;

use microlp::{ComparisonOp, OptimizationDirection, Problem};

struct Machine {
    lights: Vec<bool>,
    buttons: Vec<Vec<usize>>,
    joltage_reqs: Vec<u64>,
}

fn parse_lights(input: &str) -> IResult<&str, Vec<bool>> {
    many1(alt((value(true, char('#')), value(false, char('.'))))).parse(input)
}

fn parse_button(input: &str) -> IResult<&str, Vec<usize>> {
    separated_list1(char(','), usize).parse(input)
}

fn parse_joltage_reqs(input: &str) -> IResult<&str, Vec<u64>> {
    separated_list1(char(','), u64).parse(input)
}

fn parse_machine(input: &str) -> IResult<&str, Machine> {
    map(
        (
            delimited(char('['), parse_lights, char(']')),
            space0,
            separated_list1(space0, delimited(char('('), parse_button, char(')'))),
            space0,
            delimited(char('{'), parse_joltage_reqs, char('}')),
        ),
        |(lights, _, buttons, _, joltage_reqs)| Machine {
            lights,
            buttons,
            joltage_reqs,
        },
    )
    .parse(input)
}

fn parse_input(input: &str) -> IResult<&str, Vec<Machine>> {
    separated_list1(newline, parse_machine).parse(input)
}

fn toggle_lights(state: &Vec<bool>, button: &Vec<usize>) -> Vec<bool> {
    let mut out = state.clone();
    for &l in button {
        out[l] = !out[l];
    }
    out
}

fn bfs_lights(m: &Machine) -> usize {
    let n = m.lights.len();
    let mut frontier: BTreeSet<Vec<bool>> = BTreeSet::new();
    frontier.insert(vec![false; n]);
    for presses in 1.. {
        let mut next: BTreeSet<Vec<bool>> = BTreeSet::new();
        for s in &frontier {
            for b in &m.buttons {
                let x = toggle_lights(&s, &b);
                if x == m.lights {
                    return presses;
                }
                next.insert(x);
            }
        }
        frontier = next;
    }
    0
}

fn optimize_joltage(machine: &Machine) -> f64 {
    let mut p = Problem::new(OptimizationDirection::Minimize);
    let vars = (0..machine.buttons.len())
        .map(|_| p.add_integer_var(1.0, (0, i32::MAX)))
        .collect_vec();
    for (n, j) in machine.joltage_reqs.iter().enumerate() {
        let lhs = machine
            .buttons
            .iter()
            .enumerate()
            .filter_map(|(m, v)| if v.iter().contains(&n) { Some(m) } else { None })
            .map(|m| (vars[m], 1.0));
        p.add_constraint(lhs, ComparisonOp::Eq, *j as f64);
    }
    p.solve().unwrap().objective()
}

fn main() -> Result<(), Box<dyn Error>> {
    let stdin = std::io::stdin();
    let mut stdin = stdin.lock();
    let mut input = String::new();

    stdin.read_to_string(&mut input)?;
    let (_, machines) = parse_input(&input).map_err(|e| format!("Invalid input: {e}"))?;

    let answer: usize = machines.iter().map(bfs_lights).sum();

    println!("Minimum number of presses to configure indicator lights: {answer}");

    let answer: f64 = machines.iter().map(optimize_joltage).sum();

    println!("Minimum number of presses to satisfy joltage requirements: {answer}");

    return Ok(());
}
