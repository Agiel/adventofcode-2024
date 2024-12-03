use aocd::*;
use regex::Regex;

#[aocd(2024, 3)]
fn main() {
    let input = input!();
    let sum = solve(&input);
    dbg!(sum);
}

fn solve(input: &str) -> u32 {
    let re = Regex::new(r"do\(\)|don't\(\)|mul\((\d+),(\d+)\)").unwrap();
    let mut sum = 0;
    let mut exec = true;
    re.captures_iter(input).for_each(|c| match c[0].as_ref() {
        "do()" => exec = true,
        "don't()" => exec = false,
        _ if exec => sum += c[1].parse::<u32>().unwrap() * c[2].parse::<u32>().unwrap(),
        _ => (),
    });
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = include_str!("./example2.txt");
        assert_eq!(solve(input), 48);
    }
}
