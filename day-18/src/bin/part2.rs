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

fn solve(input: &str, end: (i32, i32), bytes: usize) -> (i32, i32) {
    let corrupted = parse(&input);
    let mut corrupted_set = corrupted.iter().take(bytes).collect::<BTreeSet<_>>();

    let start = (0, 0);

    let mut byte = bytes - 1;

    loop {
        let mut heap = BinaryHeap::new();
        heap.push((0, start));

        let mut seen = BTreeSet::new();

        let mut path_found = false;

        while let Some((cost, pos)) = heap.pop() {
            if pos == end {
                // BinaryHeap is a max heap so cost is inverted
                path_found = true;
                break;
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
                if seen.contains(&next) || corrupted_set.contains(&next) {
                    return;
                }

                heap.push((cost - 1, next));
            });
        }
        if !path_found {
            return corrupted[byte];
        }
        byte += 1;
        corrupted_set.insert(&corrupted[byte]);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = include_str!("./example1.txt");
        assert_eq!(solve(input, (6, 6), 12), (6, 1));
    }
}
