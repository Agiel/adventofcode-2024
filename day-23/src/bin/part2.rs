use std::collections::{BTreeMap, BTreeSet};

use aocd::*;

#[aocd(2024, 23)]
fn main() {
    let input = input!();
    let sum = solve(&input);
    dbg!(sum);
}

fn parse(input: &str) -> BTreeMap<&str, BTreeSet<&str>> {
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

fn search<'a>(
    current: &str,
    members: BTreeSet<&'a str>,
    map: &'a BTreeMap<&str, BTreeSet<&str>>,
    found: &mut BTreeSet<BTreeSet<&'a str>>,
) {
    if found.contains(&members) {
        return;
    }
    found.insert(members.clone());
    map.get(current).unwrap().iter().for_each(|&neighbour| {
        if members.contains(neighbour) {
            return;
        }
        if members.is_subset(map.get(neighbour).unwrap()) {
            let mut new_members = members.clone();
            new_members.insert(neighbour);
            search(neighbour, new_members, map, found);
        }
    });
}

fn solve(input: &str) -> String {
    let map = parse(input);
    let mut found = BTreeSet::new();
    map.keys()
        .for_each(|&k| search(k, BTreeSet::from([k]), &map, &mut found));
    let mut largest = found
        .into_iter()
        .max_by_key(|s| s.len())
        .unwrap()
        .into_iter()
        .collect::<Vec<_>>();
    largest.sort();
    largest.join(",")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = include_str!("./example1.txt");
        assert_eq!(solve(input), "co,de,ka,ta");
    }
}
