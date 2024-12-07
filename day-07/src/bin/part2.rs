use aocd::*;

#[aocd(2024, 7)]
fn main() {
    let input = input!();
    let sum = solve(&input);
    dbg!(sum);
}

struct Equation {
    result: u64,
    terms: Vec<u64>,
}

fn parse(input: &str) -> Vec<Equation> {
    input
        .lines()
        .map(|line| {
            let (result, terms) = line.split_once(": ").unwrap();
            let terms = terms
                .split_whitespace()
                .map(|t| t.parse().unwrap())
                .collect();

            Equation {
                result: result.parse().unwrap(),
                terms,
            }
        })
        .collect()
}

fn test(result: u64, value: &u64, remaining: &[u64]) -> Option<u64> {
    match remaining {
        [] => (result == *value).then_some(result),
        [first, rest @ ..] => test(result, &(value + first), rest)
            .or(test(result, &(value * first), rest))
            .or_else(|| {
                let value = (value.to_string() + &first.to_string()).parse().unwrap();
                test(result, &value, rest)
            }),
    }
}

fn solve(input: &str) -> u64 {
    let equations = parse(&input);
    equations
        .iter()
        .filter_map(|eq| {
            if let [first, rest @ ..] = eq.terms.as_slice() {
                test(eq.result, first, rest)
            } else {
                None
            }
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = include_str!("./example1.txt");
        assert_eq!(solve(input), 11387);
    }
}
