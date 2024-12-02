use aocd::*;

#[aocd(2024, 2)]
fn main() {
    let input = input!();
    let sum = solve(&input);
    dbg!(sum);
}

fn solve(input: &str) -> usize {
    let reports: Vec<Vec<i32>> = input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|n| n.parse().unwrap())
                .collect()
        })
        .collect();
    reports
        .iter()
        .filter_map(|report| {
            let mut slope = report[1] - report[0];
            if slope == 0 {
                return None;
            }
            slope /= slope.abs() as i32;
            report
                .iter()
                .zip(report.iter().skip(1))
                .all(|(a, b)| {
                    let diff = (b - a) * slope;
                    diff >= 1 && diff <= 3
                })
                .then_some(())
        })
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = include_str!("./example1.txt");
        assert_eq!(solve(input), 2);
    }
}
