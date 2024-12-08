use std::collections::{BTreeMap, BTreeSet};

use aocd::*;

#[aocd(2024, 8)]
fn main() {
    let input = input!();
    let sum = solve(&input);
    dbg!(sum);
}

fn solve(input: &str) -> usize {
    let grid = input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let size = (grid[0].len() as i32, grid.len() as i32);
    let mut antennaes = BTreeMap::<char, BTreeSet<(i32, i32)>>::new();
    let mut antinodes = BTreeSet::new();
    for (y, line) in grid.iter().enumerate() {
        for (x, &c) in line.iter().enumerate() {
            match c {
                a if a.is_ascii_alphanumeric() => {
                    let this = (x as i32, y as i32);
                    if let Some(others) = antennaes.get_mut(&a) {
                        for other in others.iter() {
                            let diff = (other.0 - this.0, other.1 - this.1);
                            let mut n = 0;
                            loop {
                                let ahead = (other.0 + diff.0 * n, other.1 + diff.1 * n);
                                if ahead.0 >= 0
                                    && ahead.1 >= 0
                                    && ahead.0 < size.0
                                    && ahead.1 < size.1
                                {
                                    antinodes.insert(ahead);
                                } else {
                                    break;
                                }
                                n += 1;
                            }
                            n = 0;
                            loop {
                                let behind = (this.0 - diff.0 * n, this.1 - diff.1 * n);
                                if behind.0 >= 0
                                    && behind.1 >= 0
                                    && behind.0 < size.0
                                    && behind.1 < size.1
                                {
                                    antinodes.insert(behind);
                                } else {
                                    break;
                                }
                                n += 1;
                            }
                        }
                        others.insert(this);
                    } else {
                        antennaes.insert(a, BTreeSet::from([this]));
                    }
                }
                _ => (),
            }
        }
    }

    // for (y, line) in grid.iter().enumerate() {
    //     for (x, &c) in line.iter().enumerate() {
    //         if antinodes.contains(&(x as i32, y as i32)) {
    //             print!("#");
    //         } else {
    //             print!("{c}");
    //         }
    //     }
    //     print!("\n");
    // }

    antinodes.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = include_str!("./example1.txt");
        assert_eq!(solve(input), 34);
    }
}
