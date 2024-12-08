use std::collections::{BTreeMap, BTreeSet};

use aocd::*;

#[aocd(2024, 8)]
fn main() {
    let input = input!();
    let sum = solve(&input);
    dbg!(sum);
}

fn solve(input: &str) -> usize {
    let mut size = (0, 0);
    let mut antennaes = BTreeMap::<char, BTreeSet<(i32, i32)>>::new();
    let mut antinodes = BTreeSet::new();
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            size = (size.0.max(x + 1), size.1.max(y + 1));
            match c {
                a if a.is_ascii_alphanumeric() => {
                    let this = (x as i32, y as i32);
                    if let Some(others) = antennaes.get_mut(&a) {
                        for other in others.iter() {
                            let diff = (other.0 - this.0, other.1 - this.1);
                            antinodes.insert((other.0 + diff.0, other.1 + diff.1));
                            antinodes.insert((this.0 - diff.0, this.1 - diff.1));
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
    antinodes
        .iter()
        .filter(|n| n.0 >= 0 && n.1 >= 0 && n.0 < size.0 as i32 && n.1 < size.1 as i32)
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = include_str!("./example1.txt");
        assert_eq!(solve(input), 14);
    }
}
