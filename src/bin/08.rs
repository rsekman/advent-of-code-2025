use std::cmp::{max, min};
use std::error::Error;
use std::io::prelude::*;

use itertools::Itertools;
use nom::{
    character::complete::{char, newline, u64},
    combinator::map,
    multi::separated_list1,
    IResult, Parser,
};

use std::collections::{BTreeMap, BTreeSet};

type Coordinates = (u64, u64, u64);

fn parse_input(input: &str) -> IResult<&str, Vec<Coordinates>> {
    separated_list1(
        newline,
        map((u64, char(','), u64, char(','), u64), |(x, _, y, _, z)| {
            (x, y, z)
        }),
    )
    .parse(input)
}

fn l2_distance(&(x1, y1, z1): &Coordinates, &(x2, y2, z2): &Coordinates) -> u64 {
    x1.abs_diff(x2).pow(2) + y1.abs_diff(y2).pow(2) + z1.abs_diff(z2).pow(2)
}

fn main() -> Result<(), Box<dyn Error>> {
    let stdin = std::io::stdin();
    let mut stdin = stdin.lock();
    let mut input = String::new();

    stdin.read_to_string(&mut input)?;

    let (_, jbs) = parse_input(&input).map_err(|e| format!("Invalid input: {e}"))?;

    let mut components: Vec<BTreeSet<Coordinates>> = Vec::new();
    let mut component_for_jbs = BTreeMap::<Coordinates, usize>::new();
    for (n, &c) in jbs.iter().enumerate() {
        let mut b = BTreeSet::new();
        b.insert(c);
        components.push(b);
        component_for_jbs.insert(c, n);
    }

    let mut edges = Itertools::cartesian_product(jbs.iter(), jbs.iter())
        .filter(|(x, y)| x != y)
        .collect_vec();
    edges.sort_by_key(|(x, y)| l2_distance(x, y));

    let mut circuit_size: usize = 0;
    let mut dist: u64 = 0;
    let mut n_components = components.len();
    let max_connections = 1000;
    for (n, (a, b)) in edges.iter().step_by(2).enumerate() {
        if n == max_connections {
            let mut sizes = components.iter().map(BTreeSet::len).collect_vec();
            sizes.sort();
            circuit_size = sizes.iter().rev().take(3).product();
        }

        let (&ca, &cb) = (
            component_for_jbs.get(a).unwrap(),
            component_for_jbs.get(b).unwrap(),
        );
        if ca == cb {
            continue;
        }

        let (left, right) = components.split_at_mut(max(ca, cb));
        for &jb in right[0].iter() {
            left[min(ca, cb)].insert(jb);
            component_for_jbs.insert(jb, min(ca, cb));
        }
        right[0].clear();
        n_components -= 1;
        if n_components == 1 {
            dist = a.0 * b.0
        }
    }

    println!("The circuit size after {max_connections} connections is {circuit_size}");
    println!("The distance to the wall is {dist}");
    return Ok(());
}
