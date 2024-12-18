use std::collections::{BTreeSet, BinaryHeap};

use aocd::*;

#[aocd(2024, 18)]
fn main() {
    let input = input!();
    let sum = solve(&input, (70, 70), 1024);
    dbg!(sum);
}

fn parse(input: &str) -> Vec<(i32, i32)> {
    input
        .lines()
        .map(|l| {
            l.split_once(',')
                .map(|t| (t.0.parse().unwrap(), t.1.parse().unwrap()))
                .unwrap()
        })
        .collect()
}

fn solve(input: &str, end: (i32, i32), bytes: usize) -> i32 {
    let corrupted = parse(&input);
    let corrupted = corrupted.iter().take(bytes).collect::<BTreeSet<_>>();

    let start = (0, 0);

    let mut heap = BinaryHeap::new();
    heap.push((0, start));

    let mut seen = BTreeSet::new();

    while let Some((cost, pos)) = heap.pop() {
        if pos == end {
            // BinaryHeap is a max heap so cost is inverted
            return -cost;
        }

        if seen.contains(&pos) {
            continue;
        }

        seen.insert(pos);

        let dirs = vec![(1, 0), (0, 1), (-1, 0), (0, -1)];
        dirs.iter().for_each(|new_dir| {
            let next = (pos.0 + new_dir.0, pos.1 + new_dir.1);
            if next.0 < 0 || next.1 < 0 || next.0 > end.0 || next.1 > end.1 {
                return;
            }
            if seen.contains(&next) || corrupted.contains(&next) {
                return;
            }

            heap.push((cost - 1, next));
        });
    }

    return 0;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = include_str!("./example1.txt");
        assert_eq!(solve(input, (6, 6), 12), 22);
    }
}
