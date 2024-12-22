use std::collections::BTreeMap;

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
    let change_maps = secrets
        .map(|s| {
            let mut change_map = BTreeMap::new();
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
                if n >= 3 && !change_map.contains_key(&(a, b, c, d)) {
                    change_map.insert((a, b, c, d), ones);
                }
            }
            change_map
        })
        .collect::<Vec<_>>();

    let mut max = 0;
    for a in -9..10 {
        for b in -9..10 {
            for c in -9..10 {
                for d in -9..10 {
                    let sum = change_maps
                        .iter()
                        .filter_map(|m| m.get(&(a, b, c, d)))
                        .sum::<i32>();
                    max = max.max(sum);
                }
            }
        }
    }
    max
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
