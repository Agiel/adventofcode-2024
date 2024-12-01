use aocd::*;

#[aocd(2024, 1)]
fn main() {
    let input = input!();
    let sum = solve(&input);
    dbg!(sum);
}

fn solve(input: &str) -> u32 {
    let lists: Vec<(u32, u32)> = input
        .lines()
        .map(|line| {
            line.split_once(' ')
                .map(|(l, r)| (l.trim().parse().unwrap(), r.trim().parse().unwrap()))
                .unwrap()
        })
        .collect();
    let mut left = lists.iter().map(|&(l, _)| l).collect::<Vec<_>>();
    left.sort();
    let mut right = lists.iter().map(|&(_, r)| r).collect::<Vec<_>>();
    right.sort();
    left.iter()
        .zip(right.iter())
        .map(|(l, r)| u32::abs_diff(*l, *r))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = include_str!("./example1.txt");
        assert_eq!(solve(input), 11);
    }
}
