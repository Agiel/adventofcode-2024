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

fn find(word: &str, x: usize, y: usize, chars: &Vec<Vec<char>>) -> usize {
    let dirs: Vec<(i32, i32)> = vec![
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];
    dirs.iter()
        .filter_map(|dir| {
            let mut pos = 0;
            let mut coord = (x as i32, y as i32);
            while pos < word.len() {
                if chars[coord.1 as usize][coord.0 as usize] != word.chars().nth(pos).unwrap() {
                    return None;
                }
                pos += 1;
                coord = (coord.0 + dir.0, coord.1 + dir.1);
                if pos < word.len()
                    && (coord.0 < 0
                        || coord.1 < 0
                        || coord.0 >= chars[0].len() as i32
                        || coord.1 >= chars.len() as i32)
                {
                    return None;
                }
            }
            Some(())
        })
        .count()
}

fn solve(input: &str) -> usize {
    let chars = parse(input);

    let mut count = 0;
    for y in 0..chars.len() {
        for x in 0..chars[0].len() {
            count += find("XMAS", x, y, &chars);
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
        assert_eq!(solve(input), 18);
    }
}
