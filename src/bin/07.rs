#![feature(string_into_chars)]

use std::error::Error;
use std::io::prelude::*;

fn main() -> Result<(), Box<dyn Error>> {
    let stdin = std::io::stdin();
    let stdin = stdin.lock();

    let mut ls = stdin.lines();
    let top = ls.next().ok_or("Invalid input: no first line")??;
    let width = top.len();

    let mut splits = 0;
    let mut tachyons: Vec<usize> = vec![0; width];

    let p: usize = top.find('S').ok_or("Invalid input: no start")?;
    tachyons[p] = 1;
    for line in ls {
        let mut new_tachyons = vec![0; width];
        let line = line?;
        for (n, c) in line.into_chars().enumerate() {
            if c == '^' && tachyons[n] > 0 {
                splits += 1;
                new_tachyons[n] = 0;
                if n > 0 {
                    new_tachyons[n - 1] += tachyons[n];
                }
                if n < width - 1 {
                    new_tachyons[n + 1] += tachyons[n];
                }
            } else {
                new_tachyons[n] += tachyons[n];
            }
        }
        tachyons = new_tachyons;
    }
    println!("Number of splits: {splits}",);
    println!("Number of timelines: {}", tachyons.iter().sum::<usize>());

    return Ok(());
}
