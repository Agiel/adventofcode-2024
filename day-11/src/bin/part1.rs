use aocd::*;

#[aocd(2024, 11)]
fn main() {
    let input = input!();
    let sum = solve(&input);
    dbg!(sum);
}

fn count_stones(stones: &[u64], blinks: u32) -> usize {
    if blinks == 0 {
        return stones.len();
    }

    stones
        .iter()
        .map(|stone| match stone {
            0 => count_stones(&[1], blinks - 1),
            _ => {
                let digits = stone.ilog10() + 1;
                if digits % 2 == 0 {
                    let digits = digits / 2;
                    count_stones(
                        &[stone / 10u64.pow(digits), stone % 10u64.pow(digits)],
                        blinks - 1,
                    )
                } else {
                    count_stones(&[stone * 2024], blinks - 1)
                }
            }
        })
        .sum()
}

fn solve(input: &str) -> usize {
    let stones = input
        .split_whitespace()
        .map(|digit| digit.parse().unwrap())
        .collect::<Vec<_>>();
    count_stones(stones.as_ref(), 25)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = include_str!("./example1.txt");
        assert_eq!(solve(input), 55312);
    }
}
