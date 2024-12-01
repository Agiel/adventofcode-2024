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
    lists
        .iter()
        .map(|(l, _)| l * lists.iter().filter(|(_, r)| r == l).count() as u32)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = include_str!("./example1.txt");
        assert_eq!(solve(input), 31);
    }
}
