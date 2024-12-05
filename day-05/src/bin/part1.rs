use std::collections::{BTreeMap, BTreeSet};

use aocd::*;

#[aocd(2024, 5)]
fn main() {
    let input = input!();
    let sum = solve(&input);
    dbg!(sum);
}

fn parse(input: &str) -> (BTreeMap<u32, BTreeSet<u32>>, Vec<Vec<u32>>) {
    input
        .split_once("\n\n")
        .map(|(rules, updates)| {
            let rules =
                rules
                    .lines()
                    .fold(BTreeMap::<u32, BTreeSet<u32>>::new(), |mut map, rule| {
                        let (before, other) = rule
                            .split_once('|')
                            .map(|(b, o)| (b.parse().unwrap(), o.parse().unwrap()))
                            .unwrap();
                        map.entry(before)
                            .and_modify(|e| {
                                e.insert(other);
                            })
                            .or_insert(BTreeSet::from([other]));
                        map
                    });
            let updates = updates
                .lines()
                .map(|line| line.split(',').map(|n| n.parse().unwrap()).collect())
                .collect();
            (rules, updates)
        })
        .unwrap()
}

fn solve(input: &str) -> u32 {
    let (rules, updates) = parse(&input);

    updates
        .iter()
        .flat_map(|update| {
            let mut visited = BTreeSet::new();
            let valid = update.iter().all(|page| {
                if let Some(before) = rules.get(page) {
                    let valid = before.intersection(&visited).count() == 0;
                    visited.insert(*page);
                    valid
                } else {
                    visited.insert(*page);
                    true
                }
            });
            if valid {
                Some(update[update.len() / 2])
            } else {
                None
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
        assert_eq!(solve(input), 143);
    }
}
