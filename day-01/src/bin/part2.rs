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
                .map(|tuple| {
                    (
                        tuple.0.trim().parse().unwrap(),
                        tuple.1.trim().parse().unwrap(),
                    )
                })
                .unwrap()
        })
        .collect();
    lists
        .iter()
        .map(|tuple| tuple.0 * lists.iter().filter(|other| other.1 == tuple.0).count() as u32)
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
