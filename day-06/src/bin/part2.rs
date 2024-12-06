use std::collections::BTreeSet;

use aocd::*;

#[aocd(2024, 6)]
fn main() {
    let input = input!();
    let sum = solve(&input);
    dbg!(sum);
}

#[derive(Debug)]
struct Map {
    size: (i32, i32),
    obstacles: BTreeSet<(i32, i32)>,
    guard: (i32, i32),
}

fn parse(input: &str) -> Map {
    let mut size = (0, 0);
    let mut guard = (0, 0);
    let mut obstacles = BTreeSet::new();
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            let pos = (x as i32, y as i32);
            size = (size.0.max(pos.0 + 1), size.1.max(pos.1 + 1));
            match c {
                '#' => {
                    obstacles.insert(pos);
                }
                '^' => {
                    guard = pos;
                }
                _ => (),
            }
        }
    }
    Map {
        size,
        obstacles,
        guard,
    }
}

fn solve(input: &str) -> usize {
    let map = parse(&input);
    let dirs = vec![(0, -1), (1, 0), (0, 1), (-1, 0)];

    let mut obstacles = map.obstacles;
    let mut loops = 0;
    for y in 0..map.size.0 {
        for x in 0..map.size.1 {
            let pos = (x, y);
            if obstacles.contains(&pos) || map.guard == pos {
                continue;
            }
            obstacles.insert(pos);
            let mut steps = BTreeSet::new();
            let mut guard = map.guard;
            let mut dir = 0;
            while guard.0 >= 0 && guard.1 >= 0 && guard.0 < map.size.0 && guard.1 < map.size.1 {
                if steps.contains(&(guard, dir)) {
                    loops += 1;
                    break;
                }
                while obstacles.contains(&(guard.0 + dirs[dir].0, guard.1 + dirs[dir].1)) {
                    steps.insert((guard, dir));
                    dir = (dir + 1) % dirs.len();
                }
                guard = (guard.0 + dirs[dir].0, guard.1 + dirs[dir].1);
            }
            obstacles.remove(&pos);
        }
    }

    loops
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
