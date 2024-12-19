use aocd::*;
use regex::Regex;

#[aocd(2024, 19)]
fn main() {
    let input = input!();
    let sum = solve(&input);
    dbg!(sum);
}

fn parse(input: &str) -> (Vec<&str>, Vec<&str>) {
    let (towels, patterns) = input.split_once("\n\n").unwrap();
    (
        towels.split(", ").collect(),
        patterns.split_whitespace().collect(),
    )
}

fn solve(input: &str) -> usize {
    let (towels, patterns) = parse(input);
    let re = Regex::new(format!("^({})+$", towels.join("|")).as_str()).unwrap();
    patterns
        .iter()
        .filter_map(|p| re.is_match(p).then_some(()))
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = include_str!("./example1.txt");
        assert_eq!(solve(input), 6);
    }
}
