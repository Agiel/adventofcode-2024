use std::collections::{BTreeMap, BTreeSet};

use aocd::*;

#[aocd(2024, 23)]
fn main() {
    let input = input!();
    let sum = solve(&input);
    dbg!(sum);
}

fn parse<'a>(input: &'a str) -> BTreeMap<&'a str, BTreeSet<&'a str>> {
    let mut map = BTreeMap::<&str, BTreeSet<&str>>::new();
    input.lines().for_each(|l| {
        let (k, v) = l.split_once("-").unwrap();
        map.entry(k)
            .and_modify(|s| {
                s.insert(v);
            })
            .or_insert(BTreeSet::from([v]));
        map.entry(v)
            .and_modify(|s| {
                s.insert(k);
            })
            .or_insert(BTreeSet::from([k]));
    });
    map
}

fn solve(input: &str) -> usize {
    let map = parse(input);
    let mut triples = BTreeSet::new();
    map.keys().for_each(|k| {
        if k.starts_with("t") {
            let mut members = BTreeSet::from([k]);
            for k2 in map.get(k).unwrap() {
                members.insert(k2);
                for k3 in map.get(k2).unwrap() {
                    if k3 == k {
                        continue;
                    }
                    if !map.get(k3).unwrap().contains(k) {
                        continue;
                    }
                    members.insert(k3);
                    triples.insert(members.clone());
                    members.remove(k3);
                }
                members.remove(k2);
            }
        }
    });
    triples.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = include_str!("./example1.txt");
        assert_eq!(solve(input), 7);
    }
}
