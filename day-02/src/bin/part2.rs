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
            for n in -1..report.len() as i32 {
                let mut dampened = report.clone();
                if n >= 0 {
                    dampened.remove(n as usize);
                }

                let mut slope = dampened[1] - dampened[0];
                if slope == 0 {
                    continue;
                }
                slope /= slope.abs() as i32;
                let safe = dampened.iter().zip(dampened.iter().skip(1)).all(|(a, b)| {
                    let diff = (b - a) * slope;
                    diff >= 1 && diff <= 3
                });
                if safe {
                    return Some(());
                }
            }
            None
        })
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = include_str!("./example1.txt");
        assert_eq!(solve(input), 4);
    }
}
