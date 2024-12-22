use aocd::*;

#[aocd(2024, 22)]
fn main() {
    let input = input!();
    let sum = solve(&input);
    dbg!(sum);
}

fn next(input: u64) -> u64 {
    let input = (input ^ (input * 64)) % 16777216;
    let input = (input ^ (input / 32)) % 16777216;
    let input = (input ^ (input * 2048)) % 16777216;
    input
}

fn solve(input: &str) -> u64 {
    let secrets = input.lines().map(|s| s.parse().unwrap());
    secrets
        .map(|s| {
            let mut s = s;
            for _ in 0..2000 {
                s = next(s);
            }
            s
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = include_str!("./example1.txt");
        assert_eq!(solve(input), 37327623);
    }
}
