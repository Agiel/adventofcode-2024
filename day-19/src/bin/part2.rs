use std::collections::BTreeMap;

use aocd::*;
use regex::Regex;

#[aocd(2024, 19)]
fn main() {
    let input = input!();
    let sum = solve(&input);
    dbg!(sum);
}

fn parse(input: &str) -> (Vec<&str>, Vec<&str>) {
    let (towels, patterns) = input.split_once("\n\n").unwrap();
    (
        towels.split(", ").collect(),
        patterns.split_whitespace().collect(),
    )
}

fn combinations(towels: &Vec<&str>, pattern: &str, cache: &mut BTreeMap<String, u64>) -> u64 {
    if let Some(c) = cache.get(pattern) {
        return *c;
    }

    if pattern.len() == 0 {
        return 1;
    }
    let c = towels
        .iter()
        .map(|t| {
            if pattern.starts_with(t) {
                combinations(towels, &pattern[t.len()..], cache)
            } else {
                0
            }
        })
        .sum();
    cache.insert(pattern.to_string(), c);
    c
}

fn solve(input: &str) -> u64 {
    let (towels, patterns) = parse(input);
    let re = Regex::new(format!("^({})+$", towels.join("|")).as_str()).unwrap();
    let mut cache = BTreeMap::new();
    patterns
        .iter()
        .map(|p| {
            if re.is_match(p) {
                combinations(&towels, p, &mut cache)
            } else {
                0
            }
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = include_str!("./example1.txt");
        assert_eq!(solve(input), 16);
    }
}
