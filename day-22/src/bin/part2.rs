use std::collections::{BTreeMap, BTreeSet};

use aocd::*;

#[aocd(2024, 22)]
fn main() {
    let input = input!();
    let sum = solve(&input);
    dbg!(sum);
}

fn next(input: u64) -> u64 {
    let input = (input ^ (input * 64)) % 16777216;
    let input = (input ^ (input / 32)) % 16777216;
    let input = (input ^ (input * 2048)) % 16777216;
    input
}

fn solve(input: &str) -> i32 {
    let secrets = input.lines().map(|s| s.parse().unwrap());
    let mut change_map = BTreeMap::new();
    secrets.for_each(|s| {
        let mut seen = BTreeSet::new();
        let mut s = s;
        let mut a = 0;
        let mut b = 0;
        let mut c = 0;
        let mut d = 0;
        for n in 0..2000 {
            let prev = (s % 10) as i32;
            s = next(s);
            let ones = (s % 10) as i32;
            let change = ones - prev;
            a = b;
            b = c;
            c = d;
            d = change;
            if n >= 3 && !seen.contains(&(a, b, c, d)) {
                seen.insert((a, b, c, d));
                change_map
                    .entry((a, b, c, d))
                    .and_modify(|total| *total += ones)
                    .or_insert(ones);
            }
        }
    });

    *change_map.values().max().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = include_str!("./example2.txt");
        assert_eq!(solve(input), 23);
    }
}
