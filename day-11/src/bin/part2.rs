use std::collections::BTreeMap;

use aocd::*;

#[aocd(2024, 11)]
fn main() {
    let input = input!();
    let sum = solve(&input);
    dbg!(sum);
}

fn count_stones(stone: u64, blinks: u32, cache: &mut BTreeMap<(u64, u32), usize>) -> usize {
    if blinks == 0 {
        return 1;
    }

    if let Some(count) = cache.get(&(stone, blinks)) {
        return *count;
    }

    let count = match stone {
        0 => count_stones(1, blinks - 1, cache),
        _ => {
            let digits = stone.ilog10() + 1;
            if digits % 2 == 0 {
                let digits = digits / 2;
                count_stones(stone / 10u64.pow(digits), blinks - 1, cache)
                    + count_stones(stone % 10u64.pow(digits), blinks - 1, cache)
            } else {
                count_stones(stone * 2024, blinks - 1, cache)
            }
        }
    };

    cache.insert((stone, blinks), count);

    count
}

fn solve(input: &str) -> usize {
    let mut cache = BTreeMap::new();
    input
        .split_whitespace()
        .map(|digit| count_stones(digit.parse().unwrap(), 75, &mut cache))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = include_str!("./example1.txt");
        assert_eq!(solve(input), 65601038650482);
    }
}
