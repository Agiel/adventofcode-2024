use aocd::*;

#[aocd(2024, 9)]
fn main() {
    let input = input!();
    let sum = solve(&input);
    dbg!(sum);
}

fn parse(input: &str) -> Vec<i32> {
    let mut disk = Vec::new();
    let mut free = false;
    let mut id = 0;
    for c in input.trim().chars() {
        let len = c.to_digit(10).unwrap() as usize;
        disk.extend(std::iter::repeat_n(if free { -1 } else { id }, len));
        if !free {
            id += 1;
        }
        free = !free;
    }
    disk
}

fn solve(input: &str) -> i64 {
    let mut disk = parse(&input);
    let mut n = 0;
    let mut sum = 0;
    while n < disk.len() {
        let id = disk[n];
        if id >= 0 {
            sum += id as i64 * n as i64;
        } else {
            loop {
                let tail = disk.pop().unwrap();
                if tail >= 0 {
                    sum += tail as i64 * n as i64;
                    break;
                }
                if n >= disk.len() {
                    break;
                }
            }
        }
        n += 1;
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = include_str!("./example1.txt");
        assert_eq!(solve(input), 1928);
    }
}
