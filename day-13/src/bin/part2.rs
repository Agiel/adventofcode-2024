use aocd::*;
use regex::Regex;

#[aocd(2024, 13)]
fn main() {
    let input = input!();
    let sum = solve(&input);
    dbg!(sum);
}

#[derive(Debug)]
struct Machine {
    a: (f64, f64),
    b: (f64, f64),
    c: (f64, f64),
}

// x = (b1 * c2 - b2 * c1) / (b2 * a1 - b1 * a2)
// y = (c1 * a2 - c2 * a1) / (b2 * a1 - b1 * a2)
impl Machine {
    fn solve(&self) -> (u64, u64) {
        let solution = (
            (self.b.0 * self.c.1 - self.b.1 * self.c.0)
                / (self.b.1 * self.a.0 - self.b.0 * self.a.1),
            (self.c.0 * self.a.1 - self.c.1 * self.a.0)
                / (self.b.1 * self.a.0 - self.b.0 * self.a.1),
        );
        if solution.0.fract() != 0. || solution.1.fract() != 0. {
            (0, 0)
        } else {
            (solution.0 as u64, solution.1 as u64)
        }
    }
}

fn parse(input: &str) -> Vec<Machine> {
    input
        .split("\n\n")
        .map(|m| {
            let re = Regex::new(r"X.(\d+), Y.(\d+)").unwrap();
            let mut lines = re.captures_iter(m);
            let a = lines.next().unwrap();
            let a = (a[1].parse().unwrap(), a[2].parse().unwrap());

            let b = lines.next().unwrap();
            let b = (b[1].parse().unwrap(), b[2].parse().unwrap());

            let c = lines.next().unwrap();
            let c = (
                -c[1].parse::<f64>().unwrap() - 10_000_000_000_000.,
                -c[2].parse::<f64>().unwrap() - 10_000_000_000_000.,
            );

            Machine { a, b, c }
        })
        .collect()
}

fn solve(input: &str) -> u64 {
    let machines = parse(&input);
    machines
        .iter()
        .map(|m| {
            let solution = m.solve();
            solution.0 * 3 + solution.1
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = include_str!("./example1.txt");
        assert_eq!(solve(input), 480);
    }
}