use std::collections::BTreeSet;

use aocd::*;

#[aocd(2024, 10)]
fn main() {
    let input = input!();
    let sum = solve(&input);
    dbg!(sum);
}

fn get_score(start: (i32, i32), map: &Vec<Vec<i32>>) -> BTreeSet<(i32, i32)> {
    if map[start.1 as usize][start.0 as usize] == 9 {
        return BTreeSet::from([start]);
    } else {
        let dirs = vec![(-1, 0), (0, -1), (1, 0), (0, 1)];
        let score = dirs.iter().fold(BTreeSet::new(), |mut acc, dir| {
            let (x, y) = start;
            let (new_x, new_y) = (x + dir.0, y + dir.1);
            if new_x >= 0 && new_y >= 0 && new_x < map[0].len() as i32 && new_y < map.len() as i32 {
                if map[new_y as usize][new_x as usize] - map[y as usize][x as usize] == 1 {
                    acc.append(&mut get_score((new_x, new_y), map));
                }
            }
            acc
        });
        return score;
    }
}

fn solve(input: &str) -> usize {
    let map: Vec<Vec<i32>> = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as i32)
                .collect()
        })
        .collect();

    map.iter()
        .enumerate()
        .map(|(y, row)| {
            row.iter()
                .enumerate()
                .map(|(x, z)| {
                    if *z == 0 {
                        let score = get_score((x as i32, y as i32), &map).len();
                        score
                    } else {
                        0
                    }
                })
                .sum::<usize>()
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = include_str!("./example1.txt");
        assert_eq!(solve(input), 36);
    }
}
