use aocd::*;

#[aocd(2024, 4)]
fn main() {
    let input = input!();
    let sum = solve(&input);
    dbg!(sum);
}

fn parse(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

fn find(x: usize, y: usize, chars: &Vec<Vec<char>>) -> usize {
    if chars[y][x] != 'A' {
        return 0;
    }
    if x < 1 || y < 1 || x >= chars[0].len() - 1 || y >= chars.len() - 1 {
        return 0;
    }

    if (chars[y - 1][x - 1] == 'M' && chars[y + 1][x + 1] == 'S'
        || chars[y - 1][x - 1] == 'S' && chars[y + 1][x + 1] == 'M')
        && (chars[y + 1][x - 1] == 'M' && chars[y - 1][x + 1] == 'S'
            || chars[y + 1][x - 1] == 'S' && chars[y - 1][x + 1] == 'M')
    {
        return 1;
    }

    return 0;
}

fn solve(input: &str) -> usize {
    let chars = parse(input);

    let mut count = 0;
    for y in 0..chars.len() {
        for x in 0..chars[0].len() {
            count += find(x, y, &chars);
        }
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = include_str!("./example1.txt");
        assert_eq!(solve(input), 9);
    }
}
